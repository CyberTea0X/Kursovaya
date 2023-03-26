import { Routes, Route, Link } from 'react-router-dom';

import { Homepage } from './pages/homepage/Homepage';
import { ArtistProfile } from './pages/artistProfile/ArtistProfile';
import { Trends } from './pages/trends/Trends';
import { Profile } from './pages/register/Profile';
import { Account } from './pages/profile/Account';


//import React from 'react';
//import 'bootstrap/dist/css/bootstrap.min.css';


function App() {
  return (
    <>
      <header>
        <Link to="/">Home</Link>
        <Link to="/trends">Trends</Link>
        <Link to="/gallery">ArtistProfile</Link>
        <Link to="/pr">Profile</Link>
        <Link to="/profile">Account</Link>
      </header>
      <Routes>
        <Route path="/" element={<Homepage />} />
        <Route path="/gallery" element={<ArtistProfile />} />
        <Route path="/trends" element={<Trends />} />
        <Route path="/pr" element={<Profile />} />
        <Route path="/profile" element={<Account />} />
      </Routes>
    </>
  );
}

export default App;
