#include "order_server.h"
#include <iostream>
#include "common/nlohmann/json.hpp"

namespace Exchange {

  using json = nlohmann::json;

  OrderServer::OrderServer(ClientRequestLFQueue *client_requests, ClientResponseLFQueue *client_responses)
      :outgoing_responses_(client_responses), logger_("exchange_order_server.log"), fifo_sequencer_(client_requests, &logger_),
      orderRabbit("order", "exch", "exchange_order_server_rabbitmq.log", 
        [this](const AMQP::Message &message, uint64_t deliveryTag, bool redelivered) {
            // work with JSON here. decode JSON data comming in and return
          std::string messageBody(reinterpret_cast<const char *>(message.body()), message.bodySize());
          json jsonData = json::parse(messageBody);
          // add trade modify
          ClientRequestType req = (jsonData["op"] == "TRADE-NEW") ? ClientRequestType::NEW : (jsonData["op"] == "TRADE-CANCEL") ? ClientRequestType::CANCEL : ClientRequestType::INVALID;
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
            logger_.log("%:% %() % Received % % \n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), request.seq_num_, redelivered);
            fifo_sequencer_.addClientRequest(getCurrentNanos(), me_request);
            // acknowledge the message
            this->orderRabbit.chanel.ack(deliveryTag);
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
        ),
        responsesRabbit("responses", "exch", "exchange_order_server_response_rabbitmq.log", 
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
        ){}

  OrderServer::~OrderServer() {
    stop();

    using namespace std::literals::chrono_literals;
    std::this_thread::sleep_for(1s);
  }

  auto OrderServer::run() noexcept {
    logger_.log("%:% %() %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_));


    while (run_) {

      for (auto client_response = outgoing_responses_->getNextToRead(); outgoing_responses_->size() && client_response; client_response = outgoing_responses_->getNextToRead()) {
        auto &next_outgoing_seq_num = cid_next_outgoing_seq_num_[client_response->client_id_];
        logger_.log("%:% %() % Processing cid:% seq:% %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_),
                    client_response->client_id_, next_outgoing_seq_num, client_response->toString());

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
        const char* message = json_str.c_str();
        publish(message, strlen(message) + 1);

        outgoing_responses_->updateReadIndex();

        ++next_outgoing_seq_num;
      }
    }
  }

  void OrderServer::publish(const char *message, size_t len) {
    // send rabbit mq messag
    std::string exchange = "exch";
    std::string_view exch_view = exchange;
    std::string key = "responses";
    std::string_view key_view = key;

    // create a json message here 
    this->responsesRabbit.chanel.publish(exch_view, key_view, message, len);
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
