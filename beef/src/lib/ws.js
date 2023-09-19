import WebSocket from "isomorphic-ws";
import ReconnectingWebSocket from "reconnecting-websocket";
import { v4 as generateUuid } from "uuid";

const heartbeatInterval = 8000;
const apiUrl = "ws://egusi:6000/socket";

const connectionTimeout = 15000;

export const connect = (
    token,
    refreshToken,
    {
      logger = () => {},
      onConnectionTaken = () => {},
      onClearTokens = () => {},
      url = apiUrl,
      fetchTimeout,
      getAuthOptions,
      waitToReconnect,
    }
  ) =>
    new Promise((resolve, reject) => {
      const socket = new ReconnectingWebSocket(url, [], {
        connectionTimeout,
        WebSocket,
      });
      const api2Send = (opcode, data, ref=false) => {
        // tmp fix
        // this is to avoid ws events queuing up while socket is closed
        // then it reconnects and fires before auth goes off
        // and you get logged out
        if (socket.readyState !== socket.OPEN) return;
  
        const raw = `{"v":"0.2.0", "op":"${opcode}","p":${JSON.stringify(data)}${
          ref ? `,"ref":"${ref}"` : ""
        }}`;
  
        socket.send(raw);
        logger("out", opcode, data, ref, raw);
      };
      const apiSend = (opcode, data, fetchId) => {
        // tmp fix
        // this is to avoid ws events queuing up while socket is closed
        // then it reconnects and fires before auth goes off
        // and you get logged out
        if (socket.readyState !== socket.OPEN) {
          return;
        }
        const raw = `{"op":"${opcode}","d":${JSON.stringify(data)}${
          fetchId ? `,"fetchId":"${fetchId}"` : ""
        }}`;
  
        socket.send(raw);
        logger("out", opcode, data, fetchId, raw);
      };
  
      const listeners = [];

      socket.addEventListener("close", (error) => {
        console.log(error);
        if (error.code === 4001) {
          socket.close();
          onClearTokens();
        } else if (error.code === 4003) {
          socket.close();
          onConnectionTaken();
        } else if (error.code === 4004) {
          socket.close();
          onClearTokens();
        }
  
        if (!waitToReconnect) reject(error);
      });
  
      socket.addEventListener("message", (e) => {
        if (e.data === `"pong"` || e.data === `pong`) {
          logger("in", "pong");
          return;
        }
  
        const message = JSON.parse(e.data);
  
        logger("in", message.op, message.d, message.fetchId, e.data);
  
        if (message.op === "auth-good") {
          const connection = {
            close: () => socket.close(),
            once: (opcode, handler) => {
              const listener = { opcode, handler };
  
              listener.handler = (...params) => {
                handler(...(params));
                listeners.splice(listeners.indexOf(listener), 1);
              };
  
              listeners.push(listener);
            },
            addListener: (opcode, handler) => {
              const listener = { opcode, handler };
  
              listeners.push(listener);
  
              return () => listeners.splice(listeners.indexOf(listener), 1);
            },
            user: message.d.user,
            send: apiSend,
            sendCast: api2Send,
            sendCall: (
              opcode,
              parameters,
              doneOpcode
            ) =>
              new Promise((resolveCall, rejectFetch) => {
                // tmp fix
                // this is to avoid ws events queuing up while socket is closed
                // then it reconnects and fires before auth goes off
                // and you get logged out
                if (socket.readyState !== socket.OPEN) {
                  rejectFetch(new Error("websocket not connected"));
  
                  return;
                }
                const ref = !doneOpcode && generateUuid();
                let timeoutId = null;
                const unsubscribe = connection.addListener(
                  doneOpcode ?? opcode + ":reply",
                  (data, arrivedId) => {
                    if (!doneOpcode && arrivedId !== ref) return;
  
                    if (timeoutId) clearTimeout(timeoutId);
  
                    unsubscribe();
                    resolveCall(data);
                  }
                );
  
                if (fetchTimeout) {
                  timeoutId = setTimeout(() => {
                    unsubscribe();
                    rejectFetch(new Error("timed out"));
                  }, fetchTimeout);
                }
  
                api2Send(opcode, parameters, ref || undefined);
              }),
            fetch: (opcode, parameters, doneOpcode) =>
              new Promise((resolveFetch, rejectFetch) => {
                // tmp fix
                // this is to avoid ws events queuing up while socket is closed
                // then it reconnects and fires before auth goes off
                // and you get logged out
                if (socket.readyState !== socket.OPEN) {
                  rejectFetch(new Error("websocket not connected"));
  
                  return;
                }
                const fetchId = !doneOpcode && generateUuid();
                let timeoutId = null;
                const unsubscribe = connection.addListener(
                  doneOpcode ?? "fetch_done",
                  (data, arrivedId) => {
                    if (!doneOpcode && arrivedId !== fetchId) return;
  
                    if (timeoutId) clearTimeout(timeoutId);
  
                    unsubscribe();
                    resolveFetch(data);
                  }
                );
  
                if (fetchTimeout) {
                  timeoutId = setTimeout(() => {
                    unsubscribe();
                    rejectFetch(new Error("timed out"));
                  }, fetchTimeout);
                }
  
                apiSend(opcode, parameters, fetchId || undefined);
              }),
          };
  
          resolve(connection);
        } else {
          listeners
            .filter(({ opcode }) => opcode === message.op)
            .forEach((it) =>
              it.handler(message.d || message.p, message.fetchId || message.ref)
            );
        }
      });
  
      socket.addEventListener("open", () => {
        const id = setInterval(() => {
          if (socket.readyState === socket.CLOSED) {
            clearInterval(id);
          } else {
            socket.send("ping");
            logger("out", "ping");
          }
        }, heartbeatInterval);
  
        apiSend("auth", {
          ...getAuthOptions?.(),
        });
      });
    });

