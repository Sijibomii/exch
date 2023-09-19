import React, { useContext } from "react";
import { WebSocketContext } from "./WebsocketProvider";
import { useVerifyLoggedIn } from "./useVerifyIsLoggedIn";


export const WaitForWsAndAuth = ({
  children,
}) => {
  const { conn } = useContext(WebSocketContext);

  if (!useVerifyLoggedIn()) {
    return null;
  }

  if (!conn) {
    return <div className="flex">loading...</div>;
  }

  return <>{children}</>;
};