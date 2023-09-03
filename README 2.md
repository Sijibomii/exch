the rabbitmq contract:
refId, operation, data

for a new trade:
{
  refId:
  op: CANCEL, NEW, 
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
