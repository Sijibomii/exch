#include "snapshot_synthesizer.h"

namespace Exchange {
  SnapshotSynthesizer::SnapshotSynthesizer(MDPMarketUpdateLFQueue *market_updates)
      : snapshot_md_updates_(market_updates), logger_("exchange_snapshot_synthesizer.log"), order_pool_(ME_MAX_ORDER_IDS) {
    for(auto& orders : ticker_orders_)
      orders.fill(nullptr);

      Rabbits snapshotRabbit("snapshot", myCallback);
      // create a AMQP connection object
      AMQP::Address address("localhost", 5672, AMQP::Login("guest", "guest"), "/");
      AMQP::Connection connection(&snapshotRabbit, address);
      logger_.log("%:% %() Exchange connection successful.\n ", __FILE__, __LINE__, __FUNCTION__);
      std::string exchangeName = "exch";
      std::string exchangeType = "direct"; 

      // create a channel
      AMQP::Channel* cha;
      cha = new AMQP::Channel(&connection);
      AMQP::Channel& channel = *cha;
      AMQP::ExchangeType exchangeType = AMQP::direct; 
      int flags = AMQP::durable; 
      AMQP::Table arguments; 

      channel.declareExchange(exchangeName)
          .onSuccess([this]() {
             logger_.log("%:% %() Exchange declaration successful.\n ", __FILE__, __LINE__, __FUNCTION__);
          })
          .onError([this](const char *message) {
            logger_.log("%:% %() Exchange declaration error % \n ", __FILE__, __LINE__, __FUNCTION__, message);
          });
      channel.declareQueue(snapshotRabbit.QUEUE_NAME);
      channel.bindQueue(exchangeName, snapshotRabbit.QUEUE_NAME, snapshotRabbit.QUEUE_NAME);
      // chan = &channel;
  }

  SnapshotSynthesizer::~SnapshotSynthesizer() {
    stop();
  }

  /// Start and stop the snapshot synthesizer thread.
  void SnapshotSynthesizer::start() {
    run_ = true;
    ASSERT(Common::createAndStartThread(-1, "Exchange/SnapshotSynthesizer", [this]() { run(); }) != nullptr,
           "Failed to start SnapshotSynthesizer thread.");
  } 

  void SnapshotSynthesizer::stop() {
    run_ = false;
  }

  bool myCallback(const AMQP::Message &msg) {
    // Callback implementation not needed here
    return true;
  }

  /// Process an incremental market update and update the limit order book snapshot.
  auto SnapshotSynthesizer::addToSnapshot(const MDPMarketUpdate *market_update) {
    const auto &me_market_update = market_update->me_market_update_;
    auto *orders = &ticker_orders_.at(me_market_update.ticker_id_);
    switch (me_market_update.type_) {
      case MarketUpdateType::ADD: {
        auto order = orders->at(me_market_update.order_id_);
        ASSERT(order == nullptr, "Received:" + me_market_update.toString() + " but order already exists:" + (order ? order->toString() : ""));
        orders->at(me_market_update.order_id_) = order_pool_.allocate(me_market_update);
      }
        break;
      case MarketUpdateType::MODIFY: {
        auto order = orders->at(me_market_update.order_id_);
        ASSERT(order != nullptr, "Received:" + me_market_update.toString() + " but order does not exist.");
        ASSERT(order->order_id_ == me_market_update.order_id_, "Expecting existing order to match new one.");
        ASSERT(order->side_ == me_market_update.side_, "Expecting existing order to match new one.");

        order->qty_ = me_market_update.qty_;
        order->price_ = me_market_update.price_;
      }
        break;
      case MarketUpdateType::CANCEL: {
        auto order = orders->at(me_market_update.order_id_);
        ASSERT(order != nullptr, "Received:" + me_market_update.toString() + " but order does not exist.");
        ASSERT(order->order_id_ == me_market_update.order_id_, "Expecting existing order to match new one.");
        ASSERT(order->side_ == me_market_update.side_, "Expecting existing order to match new one.");

        order_pool_.deallocate(order);
        orders->at(me_market_update.order_id_) = nullptr;
      }
        break;
      case MarketUpdateType::SNAPSHOT_START:
      case MarketUpdateType::CLEAR:
      case MarketUpdateType::SNAPSHOT_END:
      case MarketUpdateType::TRADE:
      case MarketUpdateType::INVALID:
        break;
    }

    ASSERT(market_update->seq_num_ == last_inc_seq_num_ + 1, "Expected incremental seq_nums to increase.");
    last_inc_seq_num_ = market_update->seq_num_;
  }

  /// Publish a full snapshot cycle on the snapshot multicast stream.
  auto SnapshotSynthesizer::publishSnapshot() {
    size_t snapshot_size = 0;

    // The snapshot cycle starts with a SNAPSHOT_START message and order_id_ contains the last sequence number from the incremental market data stream used to build this snapshot.
    const MDPMarketUpdate start_market_update{snapshot_size++, {MarketUpdateType::SNAPSHOT_START, last_inc_seq_num_}};
    logger_.log("%:% %() % %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), start_market_update.toString());
    
    // snapshot_socket_.send(&start_market_update, sizeof(MDPMarketUpdate));

    // Publish order information for each order in the limit order book for each instrument.
    for (size_t ticker_id = 0; ticker_id < ticker_orders_.size(); ++ticker_id) {
      const auto &orders = ticker_orders_.at(ticker_id);

      MEMarketUpdate me_market_update;
      me_market_update.type_ = MarketUpdateType::CLEAR;
      me_market_update.ticker_id_ = ticker_id;

      // We start order information for each instrument by first publishing a CLEAR message so the downstream consumer can clear the order book.
      const MDPMarketUpdate clear_market_update{snapshot_size++, me_market_update};
      logger_.log("%:% %() % %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), clear_market_update.toString());
      // snapshot_socket_.send(&clear_market_update, sizeof(MDPMarketUpdate));

      // Publish each order.
      for (const auto order: orders) {
        if (order) {
          const MDPMarketUpdate market_update{snapshot_size++, *order};
          logger_.log("%:% %() % %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), market_update.toString());
          // snapshot_socket_.send(&market_update, sizeof(MDPMarketUpdate));

        }
      }
    }

    // The snapshot cycle ends with a SNAPSHOT_END message and order_id_ contains the last sequence number from the incremental market data stream used to build this snapshot.
    const MDPMarketUpdate end_market_update{snapshot_size++, {MarketUpdateType::SNAPSHOT_END, last_inc_seq_num_}};
    logger_.log("%:% %() % %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), end_market_update.toString());
    // snapshot_socket_.send(&end_market_update);


    logger_.log("%:% %() % Published snapshot of % orders.\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), snapshot_size - 1);
  }

  /// Main method for this thread - processes incremental updates from the market data publisher, updates the snapshot and publishes the snapshot periodically.
  void SnapshotSynthesizer::run() {
    logger_.log("%:% %() %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_));
    while (run_) {
      for (auto market_update = snapshot_md_updates_->getNextToRead(); snapshot_md_updates_->size() && market_update; market_update = snapshot_md_updates_->getNextToRead()) {
        logger_.log("%:% %() % Processing %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_),
                    market_update->toString().c_str());

        addToSnapshot(market_update);

        snapshot_md_updates_->updateReadIndex();
      }

      if (getCurrentNanos() - last_snapshot_time_ > 60 * NANOS_TO_SECS) {
        last_snapshot_time_ = getCurrentNanos();
        publishSnapshot();
      }
    }
  }
  
  auto SnapshotSynthesizer::publish(const void *data, size_t len) {
    // send rabbit mq messag
    std::string exchange = "exch";
    std::string_view exch_view = exchange;
    std::string key = "snapshot";
    std::string_view key_view = key;
    this->channel.publish(exch_view, key_view, static_cast<const char*>(data), len);
  }
}