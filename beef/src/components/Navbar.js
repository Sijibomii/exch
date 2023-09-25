import { useVerifyLoggedIn } from "../lib/useVerifyIsLoggedIn";
import { Link } from "react-router-dom";
function Navbar() {

  function logOut(){
    localStorage.removeItem("@exch/token");
    localStorage.removeItem("@exch/userId");
    localStorage.removeItem("@exch/email");
    window.location.href = "/login"
  }

  return (
   <div className="bg-[#172635]">
        <div className="py-3 max-w-7xl m-auto flex items-center justify-between">
            <Link  to={'/'} className="">
              <div className="logo text-[#E3A014] text-5xl font-bold">Exch.</div>
            </Link>
            
            <div className="btns">
              {!useVerifyLoggedIn() ? (
                <>
                <a href="/login" className="text-[#E3A014] px-3">Login</a>
                <a href="/register" className="text-[#E3A014] px-3">Register</a>
                </>
              ): (
                <>
                <Link to={'/wallets'} className="text-[#E3A014] px-3">wallets</Link>
                {/* TODO: RESEARCH MORE ON THE RELOAD ISSUE AND CONVERT ALL A TAGES TO LINK */}
                <button onClick={logOut}  className="text-[#E3A014] px-3">Logout</button>
                </>
              )}
                
            </div>
        </div>
   </div>
  );
}

export default Navbar;