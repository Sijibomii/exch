#include "market_data_publisher.h"
#include "common/nlohmann/json.hpp"

namespace Exchange {
  using json = nlohmann::json;

  MarketDataPublisher::MarketDataPublisher(MEMarketUpdateLFQueue *market_updates): outgoing_md_updates_(market_updates), run_(false) {
    std::string conn_str = "guest:guest@rabbits:5672/exch";
    std::string queue = "incremental";
    std::string exchange = "exch";
    try {
        AMQP amqp(conn_str);
        AMQPExchange * ex = amqp.createExchange(exchange);
        ex->Declare(exchange, "direct");

        short my_param = AMQP_AUTODELETE | AMQP_DURABLE;
        ex->setParam(my_param);

        AMQPQueue * qu2 = amqp.createQueue(queue);

        qu2->Declare();
        qu2->Bind(exchange, queue);

        ex->setHeader("Delivery-mode", AMQP_DELIVERY_PERSISTENT);
        ex->setHeader("Content-type", "text/text");
        ex->setHeader("Content-encoding", "UTF-8");

    } catch (AMQPException &ec) {
        std::cout << ec.getMessage() << std::endl;
    }
  }


  /// Main run loop for this thread - consumes market updates from the lock free queue from the matching engine, publishes them on the incremental multicast stream and forwards them to the snapshot synthesizer.
  auto MarketDataPublisher::run() noexcept -> void {
    // logger_.log("%:% %() %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_));
    while (run_) {
    //   // for (auto market_update = outgoing_md_updates_->getNextToRead();
    //   //      outgoing_md_updates_->size() && market_update; market_update = outgoing_md_updates_->getNextToRead()) {
    //   //   // TTT_MEASURE(T5_MarketDataPublisher_LFQueue_read, logger_);

    //   //   // logger_.log("%:% %() % Sending seq:% %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), next_inc_seq_num_,
    //   //   //             market_update->toString().c_str());

    //   //   // START_MEASURE(Exchange_McastSocket_send);
        
    //   //   json jsonData;
    //   //   jsonData["refId"] = NULL;
    //   //   jsonData["op"] = "MARKET-UPDATE-" + marketUpdateTypeToString(market_update->type_);
    //   //   jsonData["data"]["seq_num"] = next_inc_seq_num_; 
    //   //   jsonData["data"]["ticker_id"] = market_update->ticker_id_;
    //   //   jsonData["data"]["order_id"] = market_update->order_id_;
    //   //   jsonData["data"]["side"] = (market_update->side_ == Side::BUY) ? "BUY" : "SELL";
    //   //   jsonData["data"]["price"] = market_update->price_;
    //   //   jsonData["data"]["qty"] = market_update->qty_;
    //   //   // publish(&next_inc_seq_num_, sizeof(next_inc_seq_num_));
    //   //   std::string json_str = jsonData.dump();
    //   //   const char* message = json_str.c_str();
    //   //   publish(message, strlen(message) + 1);
    //   //   // END_MEASURE(Exchange_McastSocket_send, logger_);

    //   //   outgoing_md_updates_->updateReadIndex();
    //   //   // TTT_MEASURE(T6_MarketDataPublisher_UDP_write, logger_);

    //   //   // Forward this incremental market data update the snapshot synthesizer.
    //   //   auto next_write = snapshot_md_updates_.getNextToWriteTo();
    //   //   next_write->seq_num_ = next_inc_seq_num_;
    //   //   next_write->me_market_update_ = *market_update;
    //   //   snapshot_md_updates_.updateWriteIndex();

    //   //   ++next_inc_seq_num_;
    //   // }
    };
  }

  void MarketDataPublisher::publish(const char *message, size_t len) {
    static_cast<void>(message);
    static_cast<void>(len);
    // send rabbit mq messag
    // std::string exchange = "exch";
    // std::string_view exch_view = exchange;
    // std::string key = "incremental";
    // std::string_view key_view = key;

    // // create a json message here 
    // this->incrementalRabbit.chanel.publish(exch_view, key_view, message, len);
  }
}

