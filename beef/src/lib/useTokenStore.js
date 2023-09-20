import create from "zustand";
import { combine } from "zustand/middleware";


const accessTokenKey = "@exch/token";

const getDefaultValues = () => {
    try {
        return {
          accessToken: localStorage.getItem(accessTokenKey) || ""
        };
    }catch {}
};

export const useTokenStore = create(
  combine(getDefaultValues(), (set) => ({
    setTokens: (x) => {
      try {
        localStorage.setItem(accessTokenKey, x.accessToken);

      } catch {}

      set(x);
    },
  }))
);