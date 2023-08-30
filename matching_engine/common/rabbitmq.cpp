#include "rabbitmq.h"
#include <amqpcpp.h>
#include <iostream>

namespace Common {

  Rabbits::Rabbits(const std::string &queue_name,  bool (*func)(const AMQP::Message &msg)): logger_("exchange_rabbit_mq.log"), QUEUE_NAME(queue_name) {}

  Rabbits::~Rabbits(){}
  // AMQP::Address address("amqp://guest:guest@localhost");
  //   MyConnectionHandler handler;

  //   AMQP::TcpConnection connection(&handler, address);
  //   connection.loop();

  //  AMQP::Address address("amqp://guest:guest@localhost/");
  //   AMQP::TcpConnection connection(address);
  //   AMQP::TcpChannel channel(&connection);

  //   channel.declareQueue("my_queue");
  //   channel.consume("my_queue")
  //       .onReceived(messageCallback)
  //       .onSuccess([](const std::string &tag) {
  //           std::cout << "Consuming from queue..." << std::endl;
  //       })
  //       .onError([](const char *message) {
  //           std::cerr << "Error while consuming: " << message << std::endl;
  //       });

  void Rabbits::onReady(AMQP::Connection *connection) {
    logger_.log("connected to rabbitmq successfully");
    // bind to a queue
  }

  void Rabbits::onError(AMQP::Connection *connection, const char *message) {
    logger_.log("error in rabbit mq: [%]", message);
  }

  void Rabbits::onClosed(AMQP::Connection *connection) {
    logger_.log("Rabbitmq connection closed");
  }

  void Rabbits::onData(AMQP::Connection *connection, const char *data, size_t size) {}



  // /// Initialize rabbitmq to read from or publish to a stream.
  // /// Does not join the rabbitmq yet.
  // auto McastSocket::init(const std::string &ip, int port) -> int {
  //   // initialize rabbitmq here
  // }

  // /// Remove / Leave membership / subscription rabbitmq.
  // auto McastSocket::leave(const std::string &, int) -> void {
  //   // stop taking messages from rabbitmq
  // } 

  // auto McastSocket::listen(const std::string& queue_name, std::function<void(const std::string&)> callback) noexcept -> bool {
  //   // registers a queue to listen to
  // }

  // /// Publish outgoing data and read incoming data.
  // auto McastSocket::dispatch() noexcept -> bool {
  //   // Read data and dispatch callbacks if data is available - non blocking.
  //   // const ssize_t n_rcv = recv(socket_fd_, inbound_data_.data() + next_rcv_valid_index_, McastBufferSize - next_rcv_valid_index_, MSG_DONTWAIT);
  //   // if (n_rcv > 0) {
  //   //   next_rcv_valid_index_ += n_rcv;
  //   //   logger_.log("%:% %() % read socket:% len:%\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), socket_fd_,
  //   //               next_rcv_valid_index_);
  //   //   recv_callback_(this);
  //   // }

  //   // Publish market data in the send buffer to the multicast stream.
  //   // if (next_send_valid_index_ > 0) {
  //   //   ssize_t n = ::send(socket_fd_, outbound_data_.data(), next_send_valid_index_, MSG_DONTWAIT | MSG_NOSIGNAL);

  //   //   logger_.log("%:% %() % send socket:% len:%\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str_), socket_fd_, n);
  //   // }
  //   // next_send_valid_index_ = 0;

  //   // return (n_rcv > 0);
  //   return false;
  // }

  /// Copy data to send buffers - does not send them out yet.
  // auto McastSocket::send(const void *data, size_t len) noexcept -> void {
  //   memcpy(outbound_data_.data() + next_send_valid_index_, data, len);
  //   next_send_valid_index_ += len;
  //   ASSERT(next_send_valid_index_ < McastBufferSize, "Mcast socket buffer filled up and sendAndRecv() not called.");
  // }
}
