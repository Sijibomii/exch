import { BrowserRouter as Router, Route, Switch,Redirect} from 'react-router-dom';
import Navbar from './components/Navbar';

function App() {
  return (
    <Router>
      <Navbar />
      <Switch>
        {/* login page */}
        {/* register page */}
        {/* all tokens page/home page */}
        {/* trade token(for a particular token) */}
      </Switch>
    </Router>
  );
}

export default App;
 