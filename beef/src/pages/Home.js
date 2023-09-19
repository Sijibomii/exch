import OfficeImage from "../images/office.avif";

const Home = () => {

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
                        <div className="flex items-center w-full justify-between px-6 py-3 border-y-white/5 border-b-2">
                            <h4 className="text-white">Token 1</h4>
                            <a href="/trade" className="bg-[#E3A014] px-1 py-1 rounded-sm">Trade</a>
                        </div>
                        <div className="flex items-center w-full justify-between px-6 py-3 border-y-white/5 border-b-2">
                            <h4 className="text-white" >Token 1</h4>
                            <a href="/trade" className="bg-[#E3A014] px-1 py-1 rounded-sm">Trade</a>
                        </div>
                        <div className="flex items-center w-full justify-between px-6 py-3 border-y-white/5 border-b-2">
                            <h4 className="text-white" >Token 1</h4>
                            <a href="/trade" className="bg-[#E3A014] px-1 py-1 rounded-sm">Trade</a>
                        </div>
                        <div className="flex items-center w-full justify-between px-6 py-3 border-y-white/5 border-b-2">
                            <h4 className="text-white" >Token 1</h4>
                            <a href="/trade" className="bg-[#E3A014] px-1 py-1 rounded-sm">Trade</a>
                        </div>
                        <div className="flex items-center w-full justify-between px-6 py-3 border-y-white/5 border-b-2">
                            <h4 className="text-white" >Token 1</h4>
                            <a href="/trade" className="bg-[#E3A014] px-1 py-1 rounded-sm">Trade</a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
      </div>
    )
}

export default Home;