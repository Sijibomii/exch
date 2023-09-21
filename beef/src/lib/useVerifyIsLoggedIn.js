import { useEffect } from "react";
import { useTokenStore } from "./useTokenStore";

export const useVerifyLoggedIn = () => {
  
  const hasTokens = useTokenStore((s) => !!(s.accessToken));

  useEffect(() => {
    if (!hasTokens && window.location.pathname !== '/login') {
        window.location = `/login`;
    }
  }, [hasTokens]);

  return hasTokens;
};