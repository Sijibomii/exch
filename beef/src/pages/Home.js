import OfficeImage from "../images/office.avif";

const Home = () => {

    return (
      <div className="home-page">
        <div className="relative">
            <div className="h-[520px] w-full">
                <img src={OfficeImage} alt="office" className="h-full w-full"/>
                <div className="overlay absolute top-0 left-0 h-full bg-black/80 w-full flex items-center justify-center">
                    <div className="details">
                        <h1 className="text-6xl text-[#E3A014] font-black">Exch. Tokens Exchange.</h1>
                    </div>
                </div>
            </div>
        </div>
      </div>
    )
}

export default Home;