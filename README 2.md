the rabbitmq contract:
refId, operation, data

sequ_num for order is different from seq_num for incremental market data publisher
for a new trade:
{
  refId:
  op: TRADE-CANCEL, TRADE-NEW, 
  data: {
    seq_num: 2,
    client_id: 1,
    ticker_id: 2,
    // order id for each client should be in sequencial order when sending order to the order-gateway.
    order_id: 3,
    side: "BUY",
    price: 10,
    qty: 200,
  }
}

OP: 
TRADE-NEW TRADE-CANCEL 
{
  refId:
  op: TRADE-CANCEL, TRADE-NEW, 
  data: {
    seq_num: 2,
    client_id: 1,
    ticker_id: 2,
    // order id for each client should be in sequencial order when sending order to the order-gateway.
    order_id: 3,
    side: "BUY",
    price: 10,
    qty: 200,
  }
}

MARKET-UPDATE-{CLEAR, ADD, MODIFY, CANCEL, TRADE, SNAPSHOT-START, SNAPSHOT-END}


USER-LOGIN
{
  refId:
  op: USER-LOGIN,
  data:{
    userId:
    email:
    trading_client_id:
    last_order_number:
    last_seq_num:
    wallet:{
      id: 
      balance
    }
  }
}

When there's a new trade, all lister get a ws message
{
 ref: UUID.uuid4(),
  op: "MARKET-UPDATE--NEW-TRADE",
  data: {
    side:
    operation: 
    volume: 
    seq_num:
    price: 
  }
}


write login for listening to udate for a ticker. i.e user session can send its id to ticker session so it get sent update from tickersession

TODO:
CONNECT RUST TO RABBITMQ AND LISTEN OR SEND MESSAGES AS APPROPRATE
HOW DOES BALANCE GET REDUCED ON CANCLE?

[
  {
    id: dnidjodjojd,
    ticker: "APPL"
  },
  {
    id: dnidjodjojd,
    ticker: "APPL"
  }
]

clone github
install docker and docker-compose
compile c++
run c++ to see 

// this is how elixir should return orderbook
time, open, close, high, low

// the incremental socket should also be in this format but close will be the current price at the moment 

////////
creating a new ticker and starting tickersession (done) -> test using postman
on trade/ddd send ws message to request orderbook and plot graph 
make sure server send regular updates
do buy and sell correctly