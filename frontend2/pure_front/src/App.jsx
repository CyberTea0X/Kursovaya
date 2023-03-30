import { HashRouter ,Routes, Route, Link } from 'react-router-dom';
import { Homepage } from './pages/homepage/Homepage';
import { ArtistProfile } from './pages/artistProfile/ArtistProfile';
import { Trends } from './pages/trends/Trends';
import { Profile } from './pages/register/Profile';

import './index.css';
import {Header} from './navbar/Navbar.jsx'






function App() {
  return (
    <>
      
        <Header />
          <Routes>
            <Route path="/" element={<Homepage />} />
            <Route path="/gallery" element={<ArtistProfile />} />
            <Route path="/trends" element={<Trends />} />
            <Route path="/Profile" element={<Profile />} />
          </Routes>
      
    </>
  );
}

export default App;
