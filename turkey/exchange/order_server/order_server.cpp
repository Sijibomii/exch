#include "order_server.h"
#include <iostream>
#include "common/nlohmann/json.hpp"

namespace Exchange {

  using json = nlohmann::json;

  OrderServer::OrderServer(ClientRequestLFQueue *client_requests, ClientResponseLFQueue *client_responses)
      :outgoing_responses_(client_responses), fifo_sequencer_(client_requests) {}

  OrderServer::~OrderServer() {
    stop();

    using namespace std::literals::chrono_literals;
    std::this_thread::sleep_for(1s);
  }

  auto OrderServer::run() noexcept {
    publish("responses");
    while (run_) {
     
      for (auto client_response = outgoing_responses_->getNextToRead(); outgoing_responses_->size() && client_response; client_response = outgoing_responses_->getNextToRead()) {
        auto &next_outgoing_seq_num = cid_next_outgoing_seq_num_[client_response->client_id_];

        // dispatch response to rabbitmq
        json jsonData;
        jsonData["refId"] = NULL;
        jsonData["op"] = "CLIENT-RESPONSE-" + clientResponseTypeToString(client_response->type_);
        jsonData["data"]["seq_num"] = next_outgoing_seq_num;
        jsonData["data"]["ticker_id"] = client_response->ticker_id_;
        jsonData["data"]["side"] = (client_response->side_ == Side::BUY) ? "BUY" : "SELL";
        jsonData["data"]["price"] = client_response->price_;
        jsonData["data"]["client_id"] = client_response->client_id_;
        // publish(&next_inc_seq_num_, sizeof(next_inc_seq_num_));
        std::string json_str = jsonData.dump();

        publish(json_str);

        outgoing_responses_->updateReadIndex();

        ++next_outgoing_seq_num;
      }
    }
  }

  auto OrderServer::run_consumer() noexcept {
    std::string conn_str = "guest:guest@rabbits:5672/exch";
    std::string queue = "responses";
    std::string exchange = "exch";
    try {
      AMQP amqp(conn_str);
      ex = amqp.createExchange(exchange);
      ex->Declare(exchange, "direct");

      short my_param = AMQP_AUTODELETE | AMQP_DURABLE;
      ex->setParam(my_param);

      // // receive from order queue
      AMQPQueue * order = amqp.createQueue("order");
      order->Declare();
      order->Bind(exchange, "order");
      order->setConsumerTag("matching_engine");
      std::function<int(AMQPMessage*)> eventCallback = [this](AMQPMessage* message) {
              return this->onMessage(message);
      };
      order->addEvent(AMQP_MESSAGE, eventCallback);
      order->Consume(AMQP_NOACK);

    } catch (AMQPException &ec) {
        std::cout << ec.getMessage() << std::endl;
    }
  }

  void OrderServer::publish(std::string message) {
    std::string conn_str = "guest:guest@rabbits:5672/exch";
    std::string queue = "responses";
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
      ex->Publish(message, "responses");
      // // receive from order queue
      // AMQPQueue * order = amqp.createQueue("order");
      // order->Declare();
      // order->Bind(exchange, "order");
      // order->setConsumerTag("matching_engine");
      // std::function<int(AMQPMessage*)> eventCallback = [this](AMQPMessage* message) {
      //         return this->onMessage(message);
      // };
      // order->addEvent(AMQP_MESSAGE, eventCallback);
      // order->addEvent(AMQP_CANCEL, onCancel);
      // order->Consume(AMQP_NOACK);

    } catch (AMQPException &ec) {
        std::cout << ec.getMessage() << std::endl;
    }
    
  }


  int OrderServer::onMessage(AMQPMessage * message) {
    (void)message;
    // uint32_t j = 0;
    // char * data = message->getMessage(&j);
    // if (data){
    //     std::string jsonString(data);
    //     json jsonData = json::parse(jsonString);
    //     // add trade modify
    //     ClientRequestType req = (jsonData["op"] == "TRADE-NEW") ? ClientRequestType::NEW : (jsonData["op"] == "TRADE-CANCEL") ? ClientRequestType::CANCEL : ClientRequestType::INVALID;
    //     size_t seq_num = jsonData["data"]["seq_num"];
    //     uint32_t client_id = jsonData["data"]["client_id"];
    //     uint32_t ticker_id = jsonData["data"]["ticker_id"]; 
    //     uint64_t order_id = jsonData["data"]["order_id"];
    //     Side side = (jsonData["data"]["side"] == "BUY") ? Side::BUY : Side::SELL;
    //     int64_t price = jsonData["data"]["price"];
    //     uint32_t qty = jsonData["data"]["qty"];
    //     // get next expected sequence number
    //     auto &next_exp_seq_num = cid_next_exp_seq_num_[client_id];
    //     if (seq_num != next_exp_seq_num) { // TODO - change this to send a reject back to the client.

    //     }else{
    //         MEClientRequest me_request{req, client_id, ticker_id, order_id, side, price, qty};
    //         OMClientRequest request {seq_num, me_request};
    //         (void)request;
    //         // logger_.log("%:% %() % Received % % \n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), request.seq_num_, redelivered);
    //         //  Exchange::fifo_sequencer_->addClientRequest(getCurrentNanos(), me_request);
    //         //  Exchange::fifo_sequencer_->sequenceAndPublish();
    //         next_exp_seq_num++;
    //     }
    // }
    return 0;
  }

  

  /// Start and stop the order server main thread.
  auto OrderServer::start() -> void {
    run_ = true;
    ASSERT(Common::createAndStartThread(-1, "Exchange/OrderServer", [this]() { run(); }) != nullptr, "Failed to start OrderServer thread.");
  }


  auto OrderServer::start_consumer() -> void {
    ASSERT(Common::createAndStartThread(-1, "Exchange/OrderServer/counsumer", [this]() { run_consumer(); }) != nullptr, "Failed to start OrderServer thread.");
  }
  auto OrderServer::stop() -> void {
    run_ = false;
  }
}
