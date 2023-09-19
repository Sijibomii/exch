import { useEffect } from "react";
import { useTokenStore } from "./useTokenStore";

export const useVerifyLoggedIn = () => {
  
  const hasTokens = useTokenStore((s) => !!(s.accessToken && s.refreshToken));

  useEffect(() => {
    if (!hasTokens) {
        window.location = `/login/?next=${window.location.href}`;
    }
  }, [hasTokens]);

  return hasTokens;
};