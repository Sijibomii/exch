import create from "zustand";
import { combine } from "zustand/middleware";


const accessTokenKey = "@exch/token";
const refreshTokenKey = "@exch/refresh-token";

const getDefaultValues = () => {
    try {
        return {
          accessToken: localStorage.getItem(accessTokenKey) || "",
          refreshToken: localStorage.getItem(refreshTokenKey) || "",
        };
    }catch {}
};

export const useTokenStore = create(
  combine(getDefaultValues(), (set) => ({
    setTokens: (x) => {
      try {
        localStorage.setItem(accessTokenKey, x.accessToken);
        localStorage.setItem(refreshTokenKey, x.refreshToken);
      } catch {}

      set(x);
    },
  }))
);