import { Routes, Route, Link } from 'react-router-dom';

import { Homepage } from './pages/Homepage';
import { Aboutpage } from './pages/Aboutpage';
import { Trends } from './pages/Trends';
import { Profile } from './pages/Profile';

//import React from 'react';
//import 'bootstrap/dist/css/bootstrap.min.css';


function App() {
  return (
    <>
      <header>
        <Link to="/">Home</Link>
        <Link to="/trends">Trends</Link>
        <Link to="/gallery">Gallery</Link>
        <Link to="/profile">Profile</Link>
      </header>
      <Routes>
        <Route path="/" element={<Homepage />} />
        <Route path="/gallery" element={<Aboutpage />} />
        <Route path="/trends" element={<Trends />} />
        <Route path="/Profile" element={<Profile />} />
      </Routes>
    </>
  );
}

export default App;
