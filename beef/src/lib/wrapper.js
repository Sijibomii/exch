const wrap = (connection) => ({
    connection,

    subscribe: {
        // market trade
        // takes a function that handles the message
        newTradeMsg: (handler) => connection.addListener("market_trade", handler),
    },


    query: {
        // get order book... take care of reply. Make sure the reply is been traslated to what the frontend expects
        getOrderBook: (
            ticker_id 
          )=> connection.fetch("all_orders", { ticker_id }),
    },


    mutation: {
        // add subscription to market trade i.e listen for market updates
        addAsListener: (ticker_id)=> connection.fetch(`schedule_room`, { ticker_id }),
        // send trade
        // BUY
        sendTrade:  (ticker_id, side, qty, price) => connection.fetch(`add_new_trade`, { ticker_id, side, qty, price }),
        // cancel trade
        cancelTrade:  (ticker_id, order_id) => connection.fetch(`cancel_trade`, { ticker_id, order_id }),
    },
});

export default wrap;