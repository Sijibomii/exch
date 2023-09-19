import { useEffect, useState, useRef, createContext, useMemo } from "react";
import { connect } from "./ws";
import { useTokenStore } from "./useTokenStore";
import { apiBaseUrl } from "./constants"

export const WebSocketContext = createContext({
    conn: null,
    setUser: () => {},
    setConn: () => {},
  });

const WebSocketProvider = ({ shouldConnect, children }) => {
    const [conn, setConn] = useState(null);
    const isConnecting = useRef(false);
    const hasTokens = useTokenStore((s) => s.accessToken && s.refreshToken);

    useEffect(() =>{
        if (!conn && shouldConnect && hasTokens && !isConnecting.current) {
            
            connect("", "", {
              waitToReconnect: true,
              url: apiBaseUrl.replace("http", "ws") + "/socket",
              getAuthOptions: () => {
                const { accessToken, refreshToken } = useTokenStore.getState();

                return {
                  accessToken,
                  refreshToken
                };
              },
              onConnectionTaken: () => {
                console.log("connection taken!");
              },
              onClearTokens: () => {
                console.log("clearing tokens...");
                useTokenStore
                  .getState()
                  .setTokens({ accessToken: "", refreshToken: "" });
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