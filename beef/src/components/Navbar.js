import { useVerifyLoggedIn } from "../lib/useVerifyIsLoggedIn";

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
            <div className="logo text-[#E3A014] text-5xl font-bold">Exch.</div>
            <div className="btns">
              {!useVerifyLoggedIn() ? (
                <>
                <a href="/login" className="text-[#E3A014] px-3">Login</a>
                <a href="/register" className="text-[#E3A014] px-3">Register</a>
                </>
              ): (
                <>
                <a href="/wallets" className="text-[#E3A014] px-3">Wallets</a>
                <button onClick={logOut}  className="text-[#E3A014] px-3">Logout</button>
                </>
              )}
                
            </div>
        </div>
   </div>
  );
}

export default Navbar;