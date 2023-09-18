#pragma once

#include <functional>
#include "common/AMQPcpp.h"
#include "market_data/snapshot_synthesizer.h"
#include "market_data/market_update.h"


namespace Exchange {
  class MarketDataPublisher {
  public:
    MarketDataPublisher(MEMarketUpdateLFQueue *market_updates);

    ~MarketDataPublisher() {
      stop();

      using namespace std::literals::chrono_literals;
      std::this_thread::sleep_for(5s);
    }

    /// Start and stop the market data publisher main thread, as well as the internal snapshot synthesizer thread.
    auto start() {
      run_ = true;
      ASSERT(Common::createAndStartThread(-1, "Exchange/MarketDataPublisher", [this]() { run(); }) != nullptr, "Failed to start MarketData thread.");
    }

    auto stop() -> void {
      run_ = false;
    }

    /// Main run loop for this thread - consumes market updates from the lock free queue from the matching engine, publishes them on the incremental multicast stream and forwards them to the snapshot synthesizer.
    auto run() noexcept -> void;

    void publish(std::string message);

    // Deleted default, copy & move constructors and assignment-operators.
    MarketDataPublisher() = delete;

    MarketDataPublisher(const MarketDataPublisher &) = delete;

    MarketDataPublisher(const MarketDataPublisher &&) = delete;

    MarketDataPublisher &operator=(const MarketDataPublisher &) = delete;

    MarketDataPublisher &operator=(const MarketDataPublisher &&) = delete;

  private:
    /// Sequencer number tracker on the incremental market data stream.
    size_t next_inc_seq_num_ = 1;

    /// Lock free queue from which we consume market data updates sent by the matching engine.
    MEMarketUpdateLFQueue *outgoing_md_updates_ = nullptr;

    volatile bool run_ = false;

    AMQPExchange * ex;

    std::string time_str_;
    
  };
}
