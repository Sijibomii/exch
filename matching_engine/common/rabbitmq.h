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


  // /// Size of send and receive buffers in bytes.
  // constexpr size_t McastBufferSize = 64 * 1024 * 1024;

  // struct McastSocket {
  //   McastSocket(Logger &logger)
  //       : logger_(logger) {
  //     outbound_data_.resize(McastBufferSize);
  //     inbound_data_.resize(McastBufferSize);
  //   }

  //   /// Initialize multicast socket to read from or publish to a stream.
  //   /// Does not join the multicast stream yet.
  //   auto init(const std::string &ip, int port) -> int;

  //   /// Remove / Leave membership / subscription to a multicast stream.
  //   auto leave(const std::string &ip, int port) -> void;

  //   /// Publish outgoing data and read incoming data.
  //   auto dispatch() noexcept -> bool;

  //   /// @brief configure queue to be listened to an monitored
  //   /// @return bool
  //   auto listen(const std::string& queue_name, std::function<void(const std::string&)> callback) noexcept -> bool;

  //   /// Copy data to send buffers - does not send them out yet.
  //   auto send(const void *data, size_t len) noexcept -> void;

  //   int socket_fd_ = -1;

  //   /// Send and receive buffers, typically only one or the other is needed, not both.
  //   std::vector<char> outbound_data_;
  //   size_t next_send_valid_index_ = 0;
  //   std::vector<char> inbound_data_;
  //   size_t next_rcv_valid_index_ = 0;

  //   std::string time_str_;
  //   Logger &logger_;
  // };
}
