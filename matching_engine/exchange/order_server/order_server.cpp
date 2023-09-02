#include "order_server.h"

namespace Exchange {
  OrderServer::OrderServer(ClientRequestLFQueue *client_requests, ClientResponseLFQueue *client_responses)
      :outgoing_responses_(client_responses), logger_("exchange_order_server.log"), fifo_sequencer_(client_requests, &logger_) {
      
      // rabbitmq
      RabbitHandler incrementalRabbit("order", "exch", "exchange_order_server_rabbitmq.log", 
        [this](const AMQP::Message &message, uint64_t deliveryTag, bool redelivered) {
            if (message.bodySize() >= sizeof(OMClientRequest)) {
            auto request = reinterpret_cast<const OMClientRequest *>(message.body());
            logger_.log("%:% %() % Received % % \n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), request->toString(), redelivered);
            fifo_sequencer_.addClientRequest(getCurrentNanos(), request->me_client_request_);
            // acknowledge the message
            this->channel.ack(deliveryTag);
            fifo_sequencer_.sequenceAndPublish();
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
