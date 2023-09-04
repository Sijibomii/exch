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
    // order id for each client should be in sequencial order
    order_id: 3,
    side: "BUY",
    price: 10,
    qty: 200,
  }
}

OP: TRADE-NEW TRADE-CANCEL MARKET-INCREMENTAL-UPDATE MARKET-SNAPSHOT

write login for listening to udate for a ticker. i.e user session can send its id to ticker session so it get sent update from tickersession