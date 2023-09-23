const axios = require("axios");
const { connect, wrap } = require("./ws");
const BASE_URL = "http://localhost:4001";

const run = async () => {
    const socketUrl = "ws://localhost:6000/socket";
    // register a user 1
    await axios({
        url: `${BASE_URL}${"/api/v1/register"}`,
        method: "POST",
        headers: { "Content-Type": "application/json" },
        data: { 
            email: "aklex1@gmail.com",
            password: "123456"
        }
    });

    setTimeout(() => {}, 2000);
    // login user 1
    // resp.data?.token
    const resp1 = await axios({
        url: `${BASE_URL}${"/api/v1/login"}`,
        method: "POST",
        headers: { "Content-Type": "application/json" },
        data: { 
            email: "aklex1@gmail.com",
            password: "123456"
        }
    });
    // console.log(resp1)
    // register a user 2
    await axios({
        url: `${BASE_URL}${"/api/v1/register"}`,
        method: "POST",
        headers: { "Content-Type": "application/json" },
        data: { 
            email: "abxex1@gmail.com",
            password: "123456"
        }
    });

    setTimeout(() => {}, 2000);
    // login user 2
    const resp2 = await axios({
        url: `${BASE_URL}${"/api/v1/login"}`,
        method: "POST",
        headers: { "Content-Type": "application/json" },
        data: { 
            email: "abxex1@gmail.com",
            password: "123456"
        }
    });

    // register a user 3
    await axios({
        url: `${BASE_URL}${"/api/v1/register"}`,
        method: "POST",
        headers: { "Content-Type": "application/json" },
        data: { 
            email: "abzzex1@gmail.com",
            password: "123456"
        }
    });
    setTimeout(() => {}, 2000);
    // login user 3
    const resp3 = await axios({
        url: `${BASE_URL}${"/api/v1/login"}`,
        method: "POST",
        headers: { "Content-Type": "application/json" },
        data: { 
            email: "abzzex1@gmail.com",
            password: "123456"
        }
    });

    // connect to websocket on all 3 users

    // connecting for user1
    // let connectionForUser1 = await connect("", "", {
    //         waitToReconnect: true,
    //         url: socketUrl,
    //         getAuthOptions: () => {

    //         return {
    //             accessToken: resp1.data.token
    //         };
    //         },
    //         onConnectionTaken: () => {
    //         console.log("connection taken!");
    //         },
    //         onClearTokens: () => {
    //         console.log("clearing tokens...");
    //         },
    //     });

    // console.log(connectionForUser1);

    // connecting for user 2
    // let connectionForUser2 = await connect("", "", {
    //     waitToReconnect: true,
    //     url: socketUrl,
    //     getAuthOptions: () => {

    //       return {
    //         accessToken: resp2.data.token
    //       };
    //     },
    //     onConnectionTaken: () => {
    //       console.log("connection taken!");
    //     },
    //     onClearTokens: () => {
    //       console.log("clearing tokens...");
    //     },
    //   });

    // connecting for user 3
    let connectionForUser3 = await connect("", "", {
        waitToReconnect: true,
        url: socketUrl,
        getAuthOptions: () => {

          return {
            accessToken: resp3.data.token
          };
        },
        onConnectionTaken: () => {
          console.log("connection taken!");
        },
        onClearTokens: () => {
          console.log("clearing tokens...");
        },
      });

    // create a new token by user 1
    const respToken = await axios({
        url: `${BASE_URL}${"/api/v1/token"}`,
        method: "POST",
        headers: { 
            "Content-Type": "application/json",
            "Authorization": `Bearer ${resp1.data.token}`
        },
        data: { 
            ticker: "KTH",
            supply: 100,
        }
    });
    
    // console.log(respToken)

    // create wallet for user 1
    const respWallet1 = await axios({
      url: `${BASE_URL}${"/api/v1/wallet"}`,
      method: "POST",
      headers: { 
          "Content-Type": "application/json",
          "Authorization": `Bearer ${resp3.data.token}`
      },
      data: {}
    });

    // fund wallet
    const respFundWallet1 = await axios({
      url: `${BASE_URL}${`/api/v1/wallet/${respWallet1.data.wallet.id}/fund`}`,
      method: "POST",
      headers: { 
          "Content-Type": "application/json",
          "Authorization": `Bearer ${resp3.data.token}`
      },
      data: {
        id: respWallet1.data.wallet.id,
        deposit: 1000
      }
    });

    // create and fund wallet for user 2
    const respWallet2 = await axios({
      url: `${BASE_URL}${"/api/v1/wallet"}`,
      method: "POST",
      headers: { 
          "Content-Type": "application/json",
          "Authorization": `Bearer ${resp2.data.token}`
      },
      data: {}
    });

    const respFundWallet2 = await axios({
      url: `${BASE_URL}${`/api/v1/wallet/${respWallet2.data.wallet.id}/fund`}`,
      method: "POST",
      headers: { 
          "Content-Type": "application/json",
          "Authorization": `Bearer ${resp2.data.token}`
      },
      data: {
        id: respWallet2.data.wallet.id,
        deposit: 1000
      }
    });

    // listen to update from this ticker from user 3
    const wrappedConn3 = wrap(connectionForUser3);
    wrappedConn3.mutation.addAsListener(respToken.data.token.ticker_id); 

    const handler = (data, ref) => {
      console.log("GOT DATA: ", data);
      console.log("GOT REF: ", ref)
    };

    await wrappedConn3.subscribe.newTradeMsg(handler);

    // send buy and sell orders from users 1 and 2
    // const wrappedConn1 = wrap(connectionForUser1);
    // const wrappedConn2 = wrap(connectionForUser2);

    // send buy and sell orders that always match so there can be a trade.
    await wrappedConn3.mutation.sendTrade(
      respToken.data.token.ticker_id,
      "BUY",
      2,
      10
    );
    // setTimeout(() => {}, 2000);
    // await wrappedConn2.mutation.sendTrade(
    //   respToken.data.token.ticker_id,
    //   "SELL",
    //   2,
    //   10
    // );

    // await wrappedConn1.mutation.sendTrade(
    //   respToken.data.token.ticker_id,
    //   "BUY",
    //   3,
    //   12
    // );
    // setTimeout(() => {}, 2000);
    // await wrappedConn2.mutation.sendTrade(
    //   respToken.data.token.ticker_id,
    //   "SELL",
    //   3,
    //   12
    // );

    // await wrappedConn1.mutation.sendTrade(
    //   respToken.data.token.ticker_id,
    //   "BUY",
    //   4,
    //   11
    // );
    // setTimeout(() => {}, 2000);

    // await wrappedConn2.mutation.sendTrade(
    //   respToken.data.token.ticker_id,
    //   "SELL",
    //   4,
    //   11
    // );
    
    // get orderbook details from user 3
    const respOrderBook = await wrappedConn3.query.getOrderBook(respToken.data.ticker_id)
    // console.log(respOrderBook)
    // cancel some orders users 1 and 2
    while(true){

    }

};
// check what happens with balance on buy and sell
run();