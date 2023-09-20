import { create } from "zustand";
import { combine } from "zustand/middleware";
import { create as createHttp } from "./http";

export const useHttpClient = create(
  combine( 
    {
      http: createHttp({
        baseUrl: "http://localhost:4001"
      })
    },
    (set) => ({
      set,
      setHttpClient: (http) => {
        set({
          http: http
        });
      },
    })
  )
);