import { createBrowserRouter, createRoutesFromElements, Route, RouterProvider, Outlet } from 'react-router-dom';
import Footer from './components/Footer';
import Navbar from './components/Navbar';
import Home from './pages/Home';
import Login from './pages/Login';
import Register from './pages/Register';
import Trade from './pages/Trade';
function App() {

  const router = createBrowserRouter(
    createRoutesFromElements(
      <Route path='/' element={<Root />}>
          <Route index element={<Home />} />
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />
          <Route path="/trade/:ticker" element={<Trade />} />
      </Route>
    )
  )

  return (
    <RouterProvider router={router} />
  );
}

export default App;

const Root = () => {

    return (
      <div className='page bg-[#151E2D]'>
        <Navbar />
        <div className='body' id="body">
          <Outlet />
        </div>
        <Footer />
      </div>
    )
}
 