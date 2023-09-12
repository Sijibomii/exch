#include "snapshot_synthesizer.h"
#include "common/nlohmann/json.hpp"

namespace Exchange {
  using json = nlohmann::json;
  SnapshotSynthesizer::SnapshotSynthesizer(MDPMarketUpdateLFQueue *market_updates)
      : snapshot_md_updates_(market_updates), logger_("exchange_snapshot_synthesizer.log"), order_pool_(ME_MAX_ORDER_IDS),
      snapshotRabbit("snapshot", "exch", "exchange_SnapshotSynthesizer_rabbitmq.log", 
      [this](const AMQP::Message &message, uint64_t deliveryTag, bool redelivered) {
        logger_.log("%:% %() % Received % % redeliverd: %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), message.body(), deliveryTag, redelivered);   
      },
      [this](const std::string &consumertag) {
        logger_.log("%:% %() % consume operation started % ", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), consumertag);   
      },
      [this](const std::string &consumertag) {
        logger_.log("%:% %() % consume operation cancelled by the RabbitMQ server % ", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), consumertag);  
      },
      [this](const char *message) {
        logger_.log("%:% %() % consume operation cancelled by the RabbitMQ server % ", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), message);  
      }
    ) {
    for(auto& orders : ticker_orders_){ 
      orders.fill(nullptr);
    }
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

    // json
    json jsonData;
    // TODO: replace NULL with a UUID
    jsonData["refId"] = NULL;
    jsonData["op"] = "MARKET-UPDATE-" + marketUpdateTypeToString(start_market_update.me_market_update_.type_);
    // first message in sequence
    jsonData["data"]["seq_num"] = start_market_update.seq_num_;
    // order_id here is the last order after which this snapshot stream will start. if the last snapshot stopped at order 5, this starts from 6
    jsonData["data"]["ticker_id"] = NULL;
    jsonData["data"]["order_id"] = start_market_update.me_market_update_.order_id_;
    jsonData["data"]["side"] = NULL;
    jsonData["data"]["price"] = NULL;
    jsonData["data"]["qty"] = NULL;
    std::string json_str = jsonData.dump();
    const char* message = json_str.c_str();
    publish(message, strlen(message) + 1);

    // Publish order information for each order in the limit order book for each instrument.
    for (size_t ticker_id = 0; ticker_id < ticker_orders_.size(); ++ticker_id) {
      const auto &orders = ticker_orders_.at(ticker_id);

      MEMarketUpdate me_market_update;
      me_market_update.type_ = MarketUpdateType::CLEAR;
      me_market_update.ticker_id_ = ticker_id;

      // We start order information for each instrument by first publishing a CLEAR message so the downstream consumer can clear the order book.
      const MDPMarketUpdate clear_market_update{snapshot_size++, me_market_update};
      logger_.log("%:% %() % %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), clear_market_update.toString());

      // json
      json jsonData;
      // TODO: replace NULL with a UUID
      jsonData["refId"] = NULL;
      jsonData["op"] = "MARKET-UPDATE-" + marketUpdateTypeToString(clear_market_update.me_market_update_.type_);
      // first message in sequence
      jsonData["data"]["seq_num"] = clear_market_update.seq_num_;
      jsonData["data"]["ticker_id"] = clear_market_update.me_market_update_.ticker_id_;
      jsonData["data"]["order_id"] = NULL;
      jsonData["data"]["side"] = NULL;
      jsonData["data"]["price"] = NULL;
      jsonData["data"]["qty"] = NULL;
      std::string json_str = jsonData.dump();
      const char* message = json_str.c_str();
      publish(message, strlen(message) + 1);

      // Publish each order.
      for (const auto order: orders) {
        if (order) {
          const MDPMarketUpdate market_update{snapshot_size++, *order};
          logger_.log("%:% %() % %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), market_update.toString());
          // json
          json jsonData;
          // TODO: replace NULL with a UUID
          jsonData["refId"] = NULL;
          jsonData["op"] = "MARKET-UPDATE-" + marketUpdateTypeToString(market_update.me_market_update_.type_);
          // first message in sequence
          jsonData["data"]["seq_num"] = market_update.seq_num_;
          jsonData["data"]["ticker_id"] = market_update.me_market_update_.ticker_id_;
          jsonData["data"]["order_id"] = market_update.me_market_update_.order_id_;
          jsonData["data"]["side"] = (market_update.me_market_update_.side_ == Side::BUY) ? "BUY" : "SELL";
          jsonData["data"]["price"] = market_update.me_market_update_.price_;
          jsonData["data"]["qty"] = market_update.me_market_update_.price_;
          std::string json_str = jsonData.dump();
          const char* message = json_str.c_str();
          publish(message, strlen(message) + 1);
          
        }
      }
    }

    // The snapshot cycle ends with a SNAPSHOT_END message and order_id_ contains the last sequence number from the incremental market data stream used to build this snapshot.
    const MDPMarketUpdate end_market_update{snapshot_size++, {MarketUpdateType::SNAPSHOT_END, last_inc_seq_num_}};
    logger_.log("%:% %() % %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), end_market_update.toString());
    // json
    json jsonD;
    // TODO: replace NULL with a UUID
    jsonD["refId"] = NULL;
    jsonD["op"] = "MARKET-UPDATE-" + marketUpdateTypeToString(end_market_update.me_market_update_.type_);
    // first message in sequence
    jsonD["data"]["seq_num"] = end_market_update.seq_num_;
    jsonD["data"]["ticker_id"] = NULL;
    jsonD["data"]["order_id"] = NULL;
    jsonD["data"]["side"] = NULL;
    jsonD["data"]["price"] = NULL;
    jsonD["data"]["qty"] = NULL;
    std::string json_ = jsonData.dump();
    const char* msg = json_.c_str();
    publish(msg, strlen(msg) + 1);

    logger_.log("%:% %() % Published snapshot of % orders.\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), snapshot_size - 1);
  }

  /// Main method for this thread - processes incremental updates from the market data publisher, updates the snapshot and publishes the snapshot periodically.
  void SnapshotSynthesizer::run() {
    logger_.log("%:% %() %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_));
    while (run_) {
      for (auto market_update = snapshot_md_updates_->getNextToRead(); snapshot_md_updates_->size() && market_update; market_update = snapshot_md_updates_->getNextToRead()) {
        logger_.log("%:% %() % Processing %\n", __FILE__, __LINE__, __FUNCTION__, getCurrentTimeStr(&time_str_), market_update->toString().c_str());

        addToSnapshot(market_update);

        snapshot_md_updates_->updateReadIndex();
      }

      if (getCurrentNanos() - last_snapshot_time_ > 60 * NANOS_TO_SECS) {
        last_snapshot_time_ = getCurrentNanos();
        publishSnapshot();
      }
    }
  }
  
  void SnapshotSynthesizer::publish(const char *message, size_t len) {
    // send rabbit mq messag
    std::string exchange = "exch"; 
    std::string_view exch_view = exchange;
    std::string key = "snapshot";
    std::string_view key_view = key;
    this->snapshotRabbit.chanel.publish(exch_view, key_view, message, len);
  }
}
