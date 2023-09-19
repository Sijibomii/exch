export const __prod__ = process.env.NODE_ENV === "production";
export const apiBaseUrl = process.env.API_BASE_URL;
export const isStaging = process.env.IS_STAGING === "true";

export const loginNextPathKey = "@exch/login-next";