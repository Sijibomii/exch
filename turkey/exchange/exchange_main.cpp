#include <csignal>

#include "matcher/matching_engine.h"
#include "market_data/market_data_publisher.h"
#include "order_server/order_server.h"


/// Main components, made global to be accessible from the signal handler.
Common::Logger *logger = nullptr;
Exchange::MatchingEngine *matching_engine = nullptr;
Exchange::MarketDataPublisher *market_data_publisher = nullptr;
Exchange::OrderServer *order_server = nullptr;

/// Shut down gracefully on external signals to this server.
void signal_handler(int) {
  using namespace std::literals::chrono_literals;
  std::this_thread::sleep_for(10s);

  // delete logger;
  // logger = nullptr;
  delete matching_engine;
  matching_engine = nullptr;
  delete market_data_publisher;
  market_data_publisher = nullptr;
  delete order_server;
  order_server = nullptr;

  std::this_thread::sleep_for(10s);

  exit(EXIT_SUCCESS);
}


// each of the app run on a thread and each of them connect to a rabbitmq queue
int main(int, char **) {
  // logger = new Common::Logger("exchange_main.log");

  std::signal(SIGINT, signal_handler);

  const int sleep_time = 100 * 1;

  // The lock free queues to facilitate communication between order server <-> matching engine and matching engine -> market data publisher.
  Exchange::ClientRequestLFQueue client_requests(ME_MAX_CLIENT_UPDATES);
  Exchange::ClientResponseLFQueue client_responses(ME_MAX_CLIENT_UPDATES);
  Exchange::MEMarketUpdateLFQueue market_updates(ME_MAX_MARKET_UPDATES);

  std::string time_str;

  // logger->log("%:% %() % Starting Matching Engine...\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str));
  matching_engine = new Exchange::MatchingEngine(&client_requests, &client_responses, &market_updates);
  matching_engine->start(); 

  // logger->log("%:% %() % Starting Market Data Publisher...\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str));
  market_data_publisher = new Exchange::MarketDataPublisher(&market_updates);
  market_data_publisher->start();

  // logger->log("%:% %() % Starting Order Server...\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str));
  order_server = new Exchange::OrderServer(&client_requests, &client_responses);
  order_server->start();

  while (true) {
    // logger->log("%:% %() % Sleeping for a few milliseconds..\n", __FILE__, __LINE__, __FUNCTION__, Common::getCurrentTimeStr(&time_str));
    usleep(sleep_time * 1);
  }
}
