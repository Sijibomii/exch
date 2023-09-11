#pragma once

#include <functional>

#include "logging.h"
#include "amqpcpp.h"


namespace Common {

  class RabbitHandler : public AMQP::ConnectionHandler 
  { 
    public:
      RabbitHandler(std::string QUEUE_NAME, std::string EXCHANGE_NAME, std::string log_file, 
      AMQP::MessageCallback &&message_callback, AMQP::ConsumeCallback &&consume_callback, 
      AMQP::CancelCallback &&cancel_callback, AMQP::ErrorCallback &&error_callback): logger_(log_file) {
        AMQP::Address address("localhost", 5672, AMQP::Login("guest", "guest"), "/");
        AMQP::Connection connection(this, address);
        logger_.log("%:% %() Exchange connection successful.\n ", __FILE__, __LINE__, __FUNCTION__);
        // create a channel
        AMQP::Channel* channel;
        channel = new AMQP::Channel(&connection);
        AMQP::Channel& chanel = *channel;
  
        chanel.declareExchange(EXCHANGE_NAME)
            .onSuccess([this]() {
                logger_.log("%:% %() Exchange declaration successful.\n ", __FILE__, __LINE__, __FUNCTION__);
            })
            .onError([this](const char *message) {
              logger_.log("%:% %() Exchange declaration error % \n ", __FILE__, __LINE__, __FUNCTION__, message);
            });
        chanel.declareQueue(QUEUE_NAME);
        chanel.bindQueue(EXCHANGE_NAME, QUEUE_NAME, QUEUE_NAME);

        chanel.consume(QUEUE_NAME)
          .onReceived(message_callback)
          .onSuccess(consume_callback)
          .onCancelled(cancel_callback)
          .onError(error_callback);

    }

    void onReady(AMQP::Connection *connection) {
      logger_.log("%:% %()connected to rabbitmq successfully % \n ", __FILE__, __LINE__, __FUNCTION__, connection->vhost());
      // bind to a queue
    }

    void onError(AMQP::Connection *connection, const char *message) {
      logger_.log("%:% %()connected to rabbitmq successfully % % \n ", __FILE__, __LINE__, __FUNCTION__, connection->vhost(), message);
    }

    void onClosed(AMQP::Connection *connection) {
      logger_.log("%:% %()connection to rabbitmq closed % \n ", __FILE__, __LINE__, __FUNCTION__, connection->vhost());
    }

    void onData(AMQP::Connection *connection, const char *data, size_t size) {
      logger_.log("%:% %()connected to rabbitmq successfully % % %s \n ", __FILE__, __LINE__, __FUNCTION__, connection->vhost(), data, size);
    }


    RabbitHandler &operator=(const RabbitHandler &) = delete;

    RabbitHandler &operator=(const RabbitHandler &&) = delete;

    std::string QUEUE_NAME;

    AMQP::Channel chanel = NULL;
    Logger logger_;
  };

}
