#include "order_server.h"

namespace Exchange {
  OrderServer::OrderServer(ClientRequestLFQueue *client_requests, ClientResponseLFQueue *client_responses)
      :outgoing_responses_(client_responses), logger_("exchange_order_server.log"), fifo_sequencer_(client_requests, &logger_) {
      
      // rabbitmq
      Rabbits orderRabbit("order", myCallback);
      // create a AMQP connection object
      AMQP::Address address("localhost", 5672, AMQP::Login("guest", "guest"), "/");
      AMQP::Connection connection(&orderRabbit, address);
      
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
      channel.declareQueue(orderRabbit.QUEUE_NAME);
      channel.bindQueue(exchangeName, orderRabbit.QUEUE_NAME, orderRabbit.QUEUE_NAME);
  }

  bool myCallback(const AMQP::Message &msg) {
    // Callback implementation not needed here
    return true;
  }

  OrderServer::~OrderServer() {
    stop();

    using namespace std::literals::chrono_literals;
    std::this_thread::sleep_for(1s);
  }

  auto OrderServer::run() {
    logger_.log("%:% %() %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_));

    // callback function that is called when the consume operation starts
    auto startCb = [](const std::string &consumertag) {
        std::cout << "consume operation started" << std::endl;
    };

    // callback function that is called when the consume operation failed
    auto errorCb = [](const char *message) {

        std::cout << "consume operation failed" << std::endl;
    };

    // callback operation when a message was received
    auto messageCb = [this](const AMQP::Message &message, uint64_t deliveryTag, bool redelivered) {
        if (message.bodySize() >= sizeof(OMClientRequest)) {
          auto request = reinterpret_cast<const OMClientRequest *>(message.body());
          logger_.log("%:% %() % Received %\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), request->toString());
          fifo_sequencer_.addClientRequest(getCurrentNanos(), request->me_client_request_);
          // acknowledge the message
          this->channel.ack(deliveryTag);
          fifo_sequencer_.sequenceAndPublish();
        }
        
    };

    // callback that is called when the consumer is cancelled by RabbitMQ (this only happens in
    // rare situations, for example when someone removes the queue that you are consuming from)
    auto cancelledCb = [](const std::string &consumertag) {
        std::cout << "consume operation cancelled by the RabbitMQ server" << std::endl;
    };
   
    this->channel.consume("order")
    .onReceived(messageCb)
    .onSuccess(startCb)
    .onCancelled(cancelledCb)
    .onError(errorCb);

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
