

function Navbar() {
  return (
   <div className="bg-[#172635]">
        <div className="py-3 max-w-7xl m-auto flex items-center justify-between">
            <div className="logo text-[#E3A014] text-5xl font-bold">Exch.</div>
            <div className="btns">
                <a href="/login" className="text-[#E3A014] px-3">Login</a>
                <a href="/register" className="text-[#E3A014] px-3">Register</a>
            </div>
        </div>
   </div>
  );
}

export default Navbar;