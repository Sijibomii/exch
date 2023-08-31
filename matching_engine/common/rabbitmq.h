#pragma once

#include <functional>

#include "logging.h"
#include "amqpcpp.h"


namespace Common {

  class Rabbits : public AMQP::ConnectionHandler 
  { 
    public:
    // takes in queue name and the handler for messages on that queue
    Rabbits(const std::string &queue_name,  bool (*func)(const AMQP::Message &msg));

    ~Rabbits();

    using HandlerFunction = std::function<void(const AMQP::Message &msg)>;
    
    void onReady(AMQP::Connection *connection) override;

    void onError(AMQP::Connection *connection, const char *message) override;

    void onClosed(AMQP::Connection *connection) override; 

    void handleMessage(const AMQP::Message &msg);

    void onData(AMQP::Connection *connection, const char *data, size_t size) override;

    /// Deleted default, copy & move constructors and assignment-operators.
    Rabbits() = delete;

    Rabbits(const Rabbits &) = delete;

    Rabbits(const Rabbits &&) = delete;

    Rabbits &operator=(const Rabbits &) = delete;

    Rabbits &operator=(const Rabbits &&) = delete;

    std::string QUEUE_NAME;

    private:

      Logger logger_;
      
      HandlerFunction handler_;
  };

}
