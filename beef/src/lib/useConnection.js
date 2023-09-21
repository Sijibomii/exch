import wrap from "./wrapper";
import { useContext } from "react";
import { WebSocketContext } from "./WebsocketProvider";

export const useConn = () => {
  return useContext(WebSocketContext).conn;
};

export const useWrappedConn = () => {
  return wrap(useContext(WebSocketContext).conn);
};