import { Routes, Route, Link } from 'react-router-dom';
import { Homepage } from './pages/homepage/Homepage';
import { ArtistProfile } from './pages/artistProfile/ArtistProfile';
import { Trends } from './pages/trends/Trends';
import { Signup } from './pages/sign-up/signup';
import { SearchPage } from './pages/search/Search.jsx';
import './index.css';
import {Header} from './navbar/Navbar.jsx';
import { Log_in } from './pages/log_in/Log_in';
import Chat from './pages/chat/chat';
import {Account} from './pages/account/Account.jsx'


function App() {
  return (
    <>
      
        <Header />
          <Routes>
            <Route path="/" element={<Homepage />} />
            <Route path="/Gallery" element={<ArtistProfile />} />
            <Route path="/Trends" element={<Trends />} />
            <Route path="/Signup" element={<Signup />} />
            <Route path="/Search" element={<SearchPage/>} />
            <Route path="/Login" element={<Log_in/>} />
            <Route path="/Account" element={<Account/>} />
            <Route path="/Chat" element={<Chat/>} />
          </Routes>
      
    </>
  );
}

export default App;
