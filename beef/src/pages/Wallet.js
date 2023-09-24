import { useState, useEffect } from "react";
import { useHttpClient } from "../lib/useHttpClient";
import { wrap } from "../lib/http"
import { useTokenStore } from "../lib/useTokenStore";


const Wallet = () => {
    const { accessToken } = useTokenStore.getState();
    const httpClient = useHttpClient();
    const wrappedClient = wrap(httpClient.http);
    const [wallets, setWallets] = useState([]);

    async function createWallet(){
        const { data } = await wrappedClient.createWallet(accessToken);
        setWallets(wallets.push(data.wallet));
    }

    async function fundWallet(walletId, amount){
        const { data } = await wrappedClient.fundWallet(walletId, accessToken, amount);
        const { wallet } = data;
        setWallets(wallets.map((wall) =>wall.id === wallet.id ? wallet : wall));
    }

    useEffect(() => {
        async function getWallets(){
            const { data } = await wrappedClient.getWallets(accessToken);
            if (data.wallets) {
                setWallets(data.wallets)
            }else{
                setWallets([])
            }
        }
        getWallets();
    }, [wallets, accessToken, wrappedClient])

    return (
        <div className="">
            <div className="max-w-7xl m-auto py-16 h-screen">
                <div className="">
                    <button onClick={createWallet}  className="bg-[#E3A014] px-3 text-black py-2">New Wallet</button>
                </div>

                <div className="mt-10">
                    <h3 className="text-[#E3A014] text-3xl">All Wallets</h3>

                    <div className="wallets">
                        {wallets && wallets.map((wallet) => (
                            <div className="wallet bg-[#E3A014] py-4 px-8 flex items-center justify-between my-2" key={wallet.id}>
                                <h3 className="">{wallet.id}</h3>
                                <h6 className="">$ {wallet.balance}</h6>
                                <button onClick={() => {
                                    return fundWallet(wallet.id, 1000)
                                }}  className="border-[#151E2D] border-2 px-3 text-black py-2">Fund</button>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    )
}

export default Wallet;