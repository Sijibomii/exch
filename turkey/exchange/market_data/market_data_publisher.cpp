#include "market_data_publisher.h"
#include "common/nlohmann/json.hpp"

#include <thread>
#include <chrono>

namespace Exchange {
  using json = nlohmann::json;

  MarketDataPublisher::MarketDataPublisher(MEMarketUpdateLFQueue *market_updates): outgoing_md_updates_(market_updates), run_(false) {
    
  }

  /// Main run loop for this thread - consumes market updates from the lock free queue from the matching engine, publishes them on the incremental multicast stream and forwards them to the snapshot synthesizer.
  auto MarketDataPublisher::run() noexcept -> void {
    while (run_) {
      for (auto market_update = outgoing_md_updates_->getNextToRead();
           outgoing_md_updates_->size() && market_update; market_update = outgoing_md_updates_->getNextToRead()) {
        json jsonData;
        jsonData["refId"] = NULL;
        jsonData["op"] = "MARKET-UPDATE-" + marketUpdateTypeToString(market_update->type_);
        jsonData["data"]["seq_num"] = next_inc_seq_num_; 
        jsonData["data"]["ticker_id"] = market_update->ticker_id_;
        jsonData["data"]["order_id"] = market_update->order_id_;
        jsonData["data"]["side"] = (market_update->side_ == Side::BUY) ? "BUY" : "SELL";
        jsonData["data"]["price"] = market_update->price_;
        jsonData["data"]["qty"] = market_update->qty_;
        std::string json_str = jsonData.dump();
        publish(json_str);
      
        outgoing_md_updates_->updateReadIndex();
        ++next_inc_seq_num_;
      }
    };
  }
  // i currently connect on every publish: that's because amqpcpp seems not to have a way of keeping it alive
  void MarketDataPublisher::publish(std::string message) {
     // connect to rabbit here
    std::string conn_str = "guest:guest@rabbits:5672/exch";
    std::string queue = "incremental";
    std::string exchange = "exch";
    try {
        AMQP amqp(conn_str);
        ex = amqp.createExchange(exchange);
        ex->Declare(exchange, "direct");

        short my_param = AMQP_AUTODELETE | AMQP_DURABLE;
        ex->setParam(my_param);

        AMQPQueue * qu2 = amqp.createQueue(queue);

        qu2->Declare();
        qu2->Bind(exchange, queue);

        ex->setHeader("Delivery-mode", AMQP_DELIVERY_PERSISTENT);
        ex->setHeader("Content-type", "text/text");
        ex->setHeader("Content-encoding", "UTF-8");
        
        ex->Publish(message, "incremental");
        std::cerr << "Successfully published to incremental queue" <<  std::endl;
    } catch (AMQPException &ec) {
        std::cout << ec.getMessage() << std::endl;
    }
  }
}

