import axios from "axios";

const BASE_URL = "http://localhost:4001";

export const create = (baseOpts) => {
    return { 
      request: async (
        method,
        endpoint,
        body, 
        headers 
      ) => {
        // const { baseUrl = BASE_URL } = { ...baseOpts, ...opts };
        return await axios({
            url: `${BASE_URL}${endpoint}`,
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
      }),
      createWallet: (token) => http.request("POST", "/api/v1/wallet", {}, { 
        "Authorization": `Bearer ${token}`
      }),
      getWallets: (token) => http.request("GET", "/api/v1/wallet",{}, {
      "Authorization": `Bearer ${token}`
      }),
      fundWallet: (id, token, deposit) => http.request("POST", `/api/v1/wallet/${id}/fund`, {
        id,
        deposit
      },{
        "Authorization": `Bearer ${token}`
      }),
      getTokens: ()=> http.request("GET", "/api/v1/token", {}),
      // getToken
    };
  };