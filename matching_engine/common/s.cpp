// #include "rabbitmq.h"
// #include <amqpcpp.h>
// #include <iostream>

// namespace Common {

//   Rabbits::Rabbits(const std::string &queue_name,  bool (*func)(const AMQP::Message &msg)): logger_("exchange_rabbit_mq.log"), QUEUE_NAME(queue_name) {}

//   Rabbits::~Rabbits(){}


//   void Rabbits::onReady(AMQP::Connection *connection) {
//     logger_.log("connected to rabbitmq successfully");
//     // bind to a queue
//   }

//   void Rabbits::onError(AMQP::Connection *connection, const char *message) {
//     logger_.log("error in rabbit mq: [%]", message);
//   }

//   void Rabbits::onClosed(AMQP::Connection *connection) {
//     logger_.log("Rabbitmq connection closed");
//   }

//   void Rabbits::onData(AMQP::Connection *connection, const char *data, size_t size) {}
// }
