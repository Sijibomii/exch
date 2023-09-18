cmake_minimum_required(VERSION 3.0)

project(MatchingEngine)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_COMPILER g++)
set(CMAKE_CXX_FLAGS "-std=c++2a -Wall -Wextra -Werror -Wpedantic")
set(CMAKE_VERBOSE_MAKEFILE on)


set(COMMON_DIR ${CMAKE_SOURCE_DIR}/common)
# build/CMakeFiles/amqpcpp.dir/src/libamqpcpp.so
link_directories(/app/amqpcpp/build/lib/)
# set(RABBITMQ_C_SOURCE_DIR ${COMMON_DIR}/rabbitmq-c-0.13.0)
# set(CMAKE_PREFIX_PATH ${CMAKE_SOURCE_DIR}/amqpcpp)

# Include directories for RabbitMQ-C headers

# include_directories(${RABBITMQ_C_SOURCE_DIR}/librabbitmq)
# include_directories(/usr/local/include)
# include_directories(/usr/local/)
# link_directories(/usr/local/lib)

add_subdirectory(common)
add_subdirectory(exchange)

list(APPEND LIBS libcommon)
list(APPEND LIBS libexchange)
list(APPEND LIBS pthread)
list(APPEND LIBS amqpcpp)

include_directories(${PROJECT_SOURCE_DIR})
include_directories(${PROJECT_SOURCE_DIR}/exchange)

# Specify the directory where amqpcpp library is located
add_executable(exchange_main exchange/exchange_main.cpp)
# target_include_directories(exchange_main PRIVATE /usr/local/include)
target_link_libraries(exchange_main PUBLIC ${LIBS})