import { Routes, Route } from 'react-router-dom';
import { Homepage } from './pages/homepage/Homepage';
import { Profile } from './pages/profile/Profile';
import { Trends } from './pages/trends/Trends';
import { Signup } from './pages/sign-up/signup';
import { SearchPage } from './pages/search/Search.jsx';
import './index.css';
import {Header} from './navbar/Navbar.jsx';
import { Login } from './pages/log_in/Login';
import { Messenger } from './pages/messenger/Messenger';
import {Settings} from './pages/account-settings/settings.jsx'


function App() {
  return (
    <>
      
        <Header />
          <Routes>
            <Route path="/" element={<Homepage />} />
            <Route path="/Profile/:userId" element={<Profile />} />
            <Route path="/Trends" element={<Trends />} />
            <Route path="/Signup" element={<Signup />} />
            <Route path="/Search" element={<SearchPage/>} />
            <Route path="/Login" element={<Login/>} />
            <Route path="/Settings" element={<Settings/>} />
            <Route path="/Messenger/*" element={<Messenger/>} />
          </Routes>
      
    </>
  );
}

export default App;
