import { useEffect, useState, useRef, createContext, useMemo } from "react";
import { connect } from "./ws";
import { useTokenStore } from "./useTokenStore";

// import { apiBaseUrl } from "./constants"

export const WebSocketContext = createContext({
    conn: null,
    setUser: () => {},
    setConn: () => {},
  });
  
const WebSocketProvider = ({ shouldConnect= true, children }) => {
    const [conn, setConn] = useState(null);
    const isConnecting = useRef(false);
    const hasTokens = useTokenStore((s) => s.accessToken);
    const socketUrl = "wss://localhost:6000/socket";;
    useEffect(() =>{
        if (!conn && shouldConnect && hasTokens && !isConnecting.current) {

            console.log("connecting....")
            connect("", "", {
              waitToReconnect: true,
              url: socketUrl,
              getAuthOptions: () => {
                const { accessToken } = useTokenStore.getState();
                return {
                  accessToken
                };
              },
              onConnectionTaken: () => {
                console.log("connection taken!");
              },
              onClearTokens: () => {
                console.log("clearing tokens...");
                useTokenStore
                  .getState()
                  .setTokens({ accessToken: "" });
                // replace("/logout");
              },
            })
            .then((x) => {
              setConn(x);
            })
            .catch((err) => {
              if (err.code === 4001) {
                window.location = '/login';
              }
            })
            .finally(() => {
              isConnecting.current = false;
            });
        }
      }, [conn, shouldConnect, hasTokens]);

      return (
        <WebSocketContext.Provider
          value={useMemo(
            () => ({
              conn,
              setConn,
              setUser: (u) => {
                if (conn) {
                  setConn({
                    ...conn,
                    user: u,
                  });
                }
              },
            }),
            [conn]
          )}
        >
          {children}
        </WebSocketContext.Provider>
      );
}

export default WebSocketProvider;