import { useEffect, useState } from "react";
import OfficeImage from "../images/office.avif";
import { useHttpClient } from "../lib/useHttpClient";
import { wrap } from "../lib/http"
import { useTokenStore } from "../lib/useTokenStore";
import { Link } from "react-router-dom";

const Home = () => {
    const httpClient = useHttpClient();
    const wrappedClient = wrap(httpClient.http);
    const hasTokens = useTokenStore((s) => s.accessToken);
    const [tokens, setTokens] = useState([]);
    const [render, setRender] = useState(true)

    async function getTokens(){
        const { data } = await wrappedClient.getTokens();
    
        if (data.tokens) {
            setTokens(data.tokens)
        }else{
            setTokens([])
        }
    }

    useEffect(()=> {
        if (render){
            getTokens();
        }
        setRender(false);
    }, [wrappedClient])

    return (
      <div className="home-page">
        <div className="">
            <div className="h-[520px] w-full relative">
                <img src={OfficeImage} alt="office" className="h-full w-full"/>
                <div className="overlay absolute top-0 left-0 h-full bg-black/80 w-full flex items-center justify-center">
                    <div className="details">
                        <h1 className="text-6xl text-[#E3A014] font-black">Exch. Tokens Exchange.</h1>
                    </div>
                </div>
            </div>

            {/* tokens list */}
            <div className="py-10">
                <div className="max-w-7xl m-auto w-full mt-8">
                    <div className="header flex bg-[#1E2734] w-full px-6 py-3">
                        <div className="">
                            <h3 className="text-2xl text-[#E3A014]">Tokens</h3>
                        </div>
                    </div>
                    <div className="w-full border-x-white/5 border-x-2">
                        {tokens && tokens.map((token) => (
                            <div className="flex items-center w-full justify-between px-6 py-3 border-y-white/5 border-b-2" key={token.id}>
                                <h4 className="text-white">{token.ticker}</h4>
                                {hasTokens ? (
                                    <Link to={`/trade/${token.ticker_id}`} className="bg-[#E3A014] px-1 py-1 rounded-sm">Trade</Link>
                                ):(
                                    <p className="text-white">please log in to trade this token</p>
                                )}
                                
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
      </div>
    )
}

export default Home;