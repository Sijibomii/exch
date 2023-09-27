const { Worker, isMainThread, parentPort, workerData } = require('worker_threads');
const axios = require("axios");
const { connect, wrap } = require("./ws");
const os = require("os");
const BASE_URL = "http://localhost:4001";


async function run () {
    if (!isMainThread) {
        const { email, password, ticker_id, isBuyer, priceIncrement } = workerData;
        const socketUrl = "ws://localhost:6000/socket";
        // register a user 1
        await axios({
            url: `${BASE_URL}${"/api/v1/register"}`,
            method: "POST",
            headers: { "Content-Type": "application/json" },
            data: { 
                email,
                password
            }
        });
    
        setTimeout(() => {}, 2000);
    
        // login 
        const resp = await axios({
            url: `${BASE_URL}${"/api/v1/login"}`,
            method: "POST",
            headers: { "Content-Type": "application/json" },
            data: { 
                email,
                password
            }
        });
    
        setTimeout(() => {}, 2000);
    
        const respWallet = await axios({
            url: `${BASE_URL}${"/api/v1/wallet"}`,
            method: "POST",
            headers: { 
                "Content-Type": "application/json",
                "Authorization": `Bearer ${resp.data.token}`
            },
            data: {}
          });
    
        setTimeout(() => {}, 2000);
    
        const respFundWallet = await axios({
            url: `${BASE_URL}${`/api/v1/wallet/${respWallet.data.wallet.id}/fund`}`,
            method: "POST",
            headers: { 
                "Content-Type": "application/json",
                "Authorization": `Bearer ${resp.data.token}`
            },
            data: {
              id: respWallet.data.wallet.id,
              deposit: 5000
            }
        });
    
        // connect to ws 
        const wsConnect = await connect("", "", {
            waitToReconnect: true,
            url: socketUrl,
            getAuthOptions: () => {
              return {
                accessToken: resp.data.token
              };
            },
            onConnectionTaken: () => {
              console.log("connection taken!");
            },
            onClearTokens: () => {
              console.log("clearing tokens...");
            },
        });
    
        const wrappedConn3 = wrap(wsConnect);
    
        wrappedConn3.mutation.addAsListener(ticker_id); 
    
        for(let i = 0; i < 100; i++) {
            if (isBuyer){
                await wrappedConn3.mutation.sendTrade(
                    ticker_id,
                    "BUY",
                    5,
                    (priceIncrement*i) + priceIncrement
                );
                setTimeout(() => {}, 2000);
            }else{
                await wrappedConn3.mutation.sendTrade(
                    ticker_id,
                    "SELL",
                    2,
                    (priceIncrement*i) + priceIncrement
                );
                setTimeout(() => {}, 2000);
            }
        }
    
    }
}

run();