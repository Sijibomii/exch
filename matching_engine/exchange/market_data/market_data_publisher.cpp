#include "market_data_publisher.h"

namespace Exchange {
  MarketDataPublisher::MarketDataPublisher(MEMarketUpdateLFQueue *market_updates, const std::string &iface,
                                           const std::string &snapshot_ip, int snapshot_port,
                                           const std::string &incremental_ip, int incremental_port)
      : outgoing_md_updates_(market_updates), snapshot_md_updates_(ME_MAX_MARKET_UPDATES),
        run_(false), logger_("exchange_market_data_publisher.log") {

          
          // , incremental_socket_(logger_) 
    // ASSERT(incremental_socket_.init(incremental_ip, iface, incremental_port, /*is_listening*/ false) >= 0,
    //        "Unable to create incremental mcast socket. error:" + std::string(std::strerror(errno)));

    // set up rabbit mq connection here 
    Rabbits incrementalRabbit("incremental", myCallback);
    // create a AMQP connection object
    AMQP::Address address("localhost", 5672, AMQP::Login("guest", "guest"), "/");
    AMQP::Connection connection(&incrementalRabbit, address);
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
    channel.declareQueue(incrementalRabbit.QUEUE_NAME);
    channel.bindQueue(exchangeName, incrementalRabbit.QUEUE_NAME, incrementalRabbit.QUEUE_NAME);

    snapshot_synthesizer_ = new SnapshotSynthesizer(&snapshot_md_updates_);
  }

  bool myCallback(const AMQP::Message &msg) {
    // Callback implementation not needed here
    return true;
  }

  /// Main run loop for this thread - consumes market updates from the lock free queue from the matching engine, publishes them on the incremental multicast stream and forwards them to the snapshot synthesizer.
  auto MarketDataPublisher::run() noexcept -> void {
    logger_.log("%:% %() %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_));
    while (run_) {
      for (auto market_update = outgoing_md_updates_->getNextToRead();
           outgoing_md_updates_->size() && market_update; market_update = outgoing_md_updates_->getNextToRead()) {
        TTT_MEASURE(T5_MarketDataPublisher_LFQueue_read, logger_);

        logger_.log("%:% %() % Sending seq:% %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), next_inc_seq_num_,
                    market_update->toString().c_str());

        START_MEASURE(Exchange_McastSocket_send);
        publish(&next_inc_seq_num_, sizeof(next_inc_seq_num_));
        publish(market_update, sizeof(MEMarketUpdate));
        END_MEASURE(Exchange_McastSocket_send, logger_);

        outgoing_md_updates_->updateReadIndex();
        TTT_MEASURE(T6_MarketDataPublisher_UDP_write, logger_);

        // Forward this incremental market data update the snapshot synthesizer.
        auto next_write = snapshot_md_updates_.getNextToWriteTo();
        next_write->seq_num_ = next_inc_seq_num_;
        next_write->me_market_update_ = *market_update;
        snapshot_md_updates_.updateWriteIndex();

        ++next_inc_seq_num_;
      }
    }
  }

  void MarketDataPublisher::publish(const void *data, size_t len) {
    // send rabbit mq messag
    std::string exchange = "exch";
    std::string_view exch_view = exchange;
    std::string key = "incremental";
    std::string_view key_view = key;
    this->channel.publish(exch_view, key_view, static_cast<const char*>(data), len);
  }
}
