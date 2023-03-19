import React, { Component } from 'react'
import { Container, Navbar, Nav,NavLink, Form, FormControl, Button } from 'react-bootstrap'
import logo from './logo192.png'
import {BrowserRouter as Router, Routes, Route, Link} from "react-router-dom";


import Home from '../Pages/Home'
import Trends from '../Pages/Trends'
import Profile from '../Pages/Profile'
import Contacts from '../Pages/Contacts'

export default class Header extends Component {
   render () {

        return (
         <>
            <Router>
                <Navbar bg="danger">
                    <Container>
                        
                        <Navbar.Toggle aria-controls="responsive-navbar-nav" />
                        <Navbar.Collapse id="responsive-navbar-nav">
                            <Nav className="mr-auto">
                                <NavLink as={Link} to ="/">Home </NavLink>
                                <NavLink as={Link} to ="/trends">Trends </NavLink>
                                <NavLink as={Link} to ="/profile">Profile </NavLink>
                                <NavLink as={Link} to ="/contacts">Contacts</NavLink>
                            </Nav>
                           
                        </Navbar.Collapse>
                    </Container>
                </Navbar>

                
                    <Routes>
                        
                        <Route exact path="/" element={Home} />
                        <Route exact path="/trends" element={Trends} />
                        <Route exact path="/profile" element={Profile} />
                        <Route exact path="/contacts" element={Contacts} />
                    </Routes>
                </Router>
        </>
        )
   }
 
}