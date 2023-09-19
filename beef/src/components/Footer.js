import Github from "../icons/Github";

function Footer() {
    return (
     <div className="bg-[#172635]">
        <div className="max-w-7xl m-auto flex items-center justify-between py-2">
            <div className="">
                <h2 className="logo text-[#E3A014] text-xl font-bold">Exch.</h2>
            </div>
            <div className="github">
                <a href="https://github.com/Sijibomii/exch">
                    <Github />
                </a>
            </div>
        </div>
     </div>
    );
  }
  
  export default Footer;