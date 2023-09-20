import axios from "axios";

const BASE_URL = "https://localhost:4001";

export const create = (baseOpts) => {
    return { 
      request: async (
        method,
        endpoint,
        body, 
        opts,
        headers 
      ) => {
        const { baseUrl = BASE_URL } = { ...baseOpts, ...opts };
        return await axios({
            url: `${baseUrl}${endpoint}`,
            method,
            headers: { "Content-Type": "application/json", ...headers},
            data:  body ? body : undefined
        }).then((res) => res)
      },
    };
};

export const wrap = (http) => {
    return { 
      login: (email, password) => http.request("POST", "/api/v1/login", { 
          email, password
      }),
      register: (email, password) => http.request("POST", "/api/v1/register", { 
          email, password
      })
    };
  };