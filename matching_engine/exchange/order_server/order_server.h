#pragma once

#include <functional>

#include "common/thread_utils.h"
#include "common/macros.h"
// #include "common/tcp_server.h"
#include "common/logging.h"
#include "common/rabbitmq.h"
#include "order_server/client_request.h"
#include "order_server/client_response.h"
#include "order_server/fifo_sequencer.h"

namespace Exchange {
  
  class OrderServer {
    public:
      OrderServer(ClientRequestLFQueue *client_requests, ClientResponseLFQueue *client_responses);

      ~OrderServer();

      /// Start and stop the order server main thread.
      auto start() -> void;

      auto stop() -> void;

      /// Main run loop for this thread - accepts new client connections, receives client requests from them and sends client responses to them.
      auto run() noexcept;
      
      void publish(const char *message, size_t len);
    /// Deleted default, copy & move constructors and assignment-operators.
    OrderServer() = delete;

    OrderServer(const OrderServer &) = delete;

    OrderServer(const OrderServer &&) = delete;

    OrderServer &operator=(const OrderServer &) = delete;

    OrderServer &operator=(const OrderServer &&) = delete;


      private:
        /// Lock free queue of outgoing client responses to be sent out to connected clients.
        ClientResponseLFQueue *outgoing_responses_ = nullptr;

        volatile bool run_ = false; 

        std::string time_str_;
        Logger logger_;
        AMQP::Channel channel = NULL;
        FIFOSequencer fifo_sequencer_;

        /// Hash map from ClientId -> the next sequence number to be sent on outgoing client responses.
        std::array<size_t, ME_MAX_NUM_CLIENTS> cid_next_outgoing_seq_num_;

        /// Hash map from ClientId -> the next sequence number expected on incoming client requests.
        std::array<size_t, ME_MAX_NUM_CLIENTS> cid_next_exp_seq_num_;
  };
}
