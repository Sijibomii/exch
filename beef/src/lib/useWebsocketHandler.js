import { useEffect, useContext } from "react";
import { WebSocketContext } from "./WebsocketProvider";


export const useMainWsHandler = () => {

    const { conn } = useContext(WebSocketContext);

    useEffect(()=>{
        if (!conn) {
            return;
        }

        const unsubs = [
            conn.addListener("error", (message) => {
                console.log(message);
            }),
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