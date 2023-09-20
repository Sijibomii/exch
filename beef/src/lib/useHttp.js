import { useHttpClient as useHttp } from "../global-stores/useHttpClient";
import { create } from "./http";

// return the http client 
export const useHttpClient = () => {
    const [setHttp, httpClient] = useHttp((state) => [state.setHttpClient, state.http]);

    if(httpClient === null){
        const client = create({
            baseUrl: process.env.BASE_API_URL || 'http://localhost:4001'
        })
        setHttp(client);
        return client
    }
    
    return httpClient;
};