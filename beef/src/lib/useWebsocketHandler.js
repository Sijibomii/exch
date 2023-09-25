import { useEffect, useContext } from "react";
import { WebSocketContext } from "./WebsocketProvider";
import ShowError from "./showError";
import showInfo from "./showInfo";
import { useOrderBookStore  } from "./useOrderBook";


export const useMainWsHandler = () => {

    const { conn } = useContext(WebSocketContext);

    const proccessError = (obj) => {
        const keyValuePairs = [];

        for (const key in obj) {
            if (obj.hasOwnProperty(key)) {
                keyValuePairs.push(`${key}: ${obj[key]}`);
            }
        }

        return keyValuePairs.join(', ');
    } 

    useEffect(()=>{
        if (!conn) {
            return;
        }
        // orders:all
        // incrmental
        // trade:new:reply
        const unsubs = [
            conn.addListener("error", (message) => {
                ShowError(message);
            }),

            conn.addListener("orders:all", (message) => {
                console.log(message)
                if(message !== null){
                    if (message.e){
                        
                        ShowError(proccessError(message.e))
                        return 
                    }

                    useOrderBookStore 
                        .getState()
                        .setOrderBook(message.p);
                }
            }),
            conn.addListener("trade:new:reply", (message) => {
                if(message){
                    if (message.e){
                        ShowError(proccessError(message.e))
                        return 
                    }
                    showInfo("trade successfully added!")
                }
            }),
            conn.addListener("MARKET-UPDATE-NEW-TRADE", (message) => {
                if(message){
                    if (message.e){
                        ShowError(proccessError(message.e))
                        return 
                    }   
                    showInfo("new trade came in!!!")

                    // useOrderBookStore 
                    //     .getState()
                    //     .setOrderBook(message.p);
                }
            })

        ];
        return () => {
            unsubs.forEach((u) => u());
        };
    },[conn])

}


const MainWsHandlerProvider = ({ children }) => {
    useMainWsHandler();
    return <>{children}</>;
};
export default MainWsHandlerProvider;