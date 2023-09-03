#include "order_server.h"
#include <iostream>
#include <nlohmann/json.hpp>

namespace Exchange {

  using json = nlohmann::json;

  OrderServer::OrderServer(ClientRequestLFQueue *client_requests, ClientResponseLFQueue *client_responses)
      :outgoing_responses_(client_responses), logger_("exchange_order_server.log"), fifo_sequencer_(client_requests, &logger_) {
      
      // rabbitmq
      RabbitHandler incrementalRabbit("order", "exch", "exchange_order_server_rabbitmq.log", 
        [this](const AMQP::Message &message, uint64_t deliveryTag, bool redelivered) {
            // work with JSON here. decode JSON data comming in and return
          std::string messageBody(reinterpret_cast<const char *>(message.body()), message.bodySize());
          json jsonData = json::parse(messageBody);
          ClientRequestType req = (jsonData["op"] == "NEW") ? ClientRequestType::NEW : ClientRequestType::CANCEL;
          size_t seq_num = jsonData["data"]["seq_num"];
          uint32_t client_id = jsonData["data"]["client_id"];
          uint32_t ticker_id = jsonData["data"]["ticker_id"];
          uint64_t order_id = jsonData["data"]["order_id"];
          Side side = (jsonData["data"]["side"] == "BUY") ? Side::BUY : Side::SELL;
          int64_t price = jsonData["data"]["price"];
          uint32_t qty = jsonData["data"]["qty"];
          // get next expected sequence number
          auto &next_exp_seq_num = cid_next_exp_seq_num_[client_id];
          if (seq_num != next_exp_seq_num) { // TODO - change this to send a reject back to the client.
            logger_.log("%:% %() % Incorrect sequence number. ClientId:% SeqNum expected:% received:%\n", __FILE__, __LINE__, __FUNCTION__,
                        Common::getCurrentTimeStr(&time_str_), client_id, next_exp_seq_num, seq_num);
          }else{
            MEClientRequest me_request{req, client_id, ticker_id, order_id, side, price, qty};
            OMClientRequest request {seq_num, me_request};
            logger_.log("%:% %() % Received % % \n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), request->toString(), redelivered);
            fifo_sequencer_.addClientRequest(getCurrentNanos(), request->me_client_request_);
            // acknowledge the message
            this->channel.ack(deliveryTag);
            fifo_sequencer_.sequenceAndPublish();
            next_exp_seq_num++;
          }
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
  }

  OrderServer::~OrderServer() {
    stop();

    using namespace std::literals::chrono_literals;
    std::this_thread::sleep_for(1s);
  }

  auto OrderServer::run() noexcept {
    logger_.log("%:% %() %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_));
    while (run_) {}
  }

  /// Start and stop the order server main thread.
  auto OrderServer::start() -> void {
    run_ = true;
    
    ASSERT(Common::createAndStartThread(-1, "Exchange/OrderServer", [this]() { run(); }) != nullptr, "Failed to start OrderServer thread.");
  }

  auto OrderServer::stop() -> void {
    run_ = false;
  }
}
