#include "market_data_publisher.h"


namespace Exchange {
 
  MarketDataPublisher::MarketDataPublisher(MEMarketUpdateLFQueue *market_updates)
      : outgoing_md_updates_(market_updates), snapshot_md_updates_(ME_MAX_MARKET_UPDATES),
        run_(false), logger_("exchange_market_data_publisher.log") {
          
    RabbitHandler incrementalRabbit("incremental", "exch", "exchange_market_data_publisher_rabbitmq.log", 
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
    );
    snapshot_synthesizer_ = new SnapshotSynthesizer(&snapshot_md_updates_);
  }


  /// Main run loop for this thread - consumes market updates from the lock free queue from the matching engine, publishes them on the incremental multicast stream and forwards them to the snapshot synthesizer.
  auto MarketDataPublisher::run() noexcept -> void {
    logger_.log("%:% %() %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_));
    while (run_) {
      for (auto market_update = outgoing_md_updates_->getNextToRead();
           outgoing_md_updates_->size() && market_update; market_update = outgoing_md_updates_->getNextToRead()) {
        // TTT_MEASURE(T5_MarketDataPublisher_LFQueue_read, logger_);

        logger_.log("%:% %() % Sending seq:% %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), next_inc_seq_num_,
                    market_update->toString().c_str());

        // START_MEASURE(Exchange_McastSocket_send);
        
        publish(&next_inc_seq_num_, sizeof(next_inc_seq_num_));
        publish(market_update, sizeof(MEMarketUpdate));
        // END_MEASURE(Exchange_McastSocket_send, logger_);

        outgoing_md_updates_->updateReadIndex();
        // TTT_MEASURE(T6_MarketDataPublisher_UDP_write, logger_);

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

