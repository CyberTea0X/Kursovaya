import React, { Component } from 'react'
import { Container, Navbar, Nav, Form, FormControl, Button } from 'react-bootstrap'
import logo from './logo192.png'
import {BrowserRouter as Router, Routes, Route, Link} from "react-router-dom";


import Home from '../Pages/Home'
import Trends from '../Pages/Trends'
import Profile from '../Pages/Profile'
import Gallery from '../Pages/Gallery'

export default class Header extends Component {
    render() {

        return (
            <>
                <Navbar collapseOnSelect expand="md" bg="danger" variant="dark" >
                    <Container>
                        <Navbar.Brand href="/" >
                            <img
                                src={logo}
                                height="30"
                                width="30"
                                className="d-inline-block align-top"
                                alt="Logo"
                            /> 
                        </Navbar.Brand>
                        <Navbar.Toggle aria-controls="responsive-navbar-nav" />
                        <Navbar.Collapse id="responsive-navbar-nav">
                            <Nav className="mr-auto">
                                <Nav.Link href="/" > Home</Nav.Link>
                                <Nav.Link href="/trends" > Trends</Nav.Link>
                                <Nav.Link href="/profile" > Profile</Nav.Link>
                                <Nav.Link href="/gallery" > Gallery</Nav.Link>
                            </Nav>
                            <Form inline>
                                <FormControl
                                    type="text"
                                    placeholder="Search"
                                    className="mr-sm-2"
                                />
                                <Button variant="outline-info">Search</Button>
                            </Form>
                        </Navbar.Collapse>
                    </Container>
                </Navbar>

                <Router>
                    <Routes>
                        <Route exact path="/" element={Home} />
                        <Route exact path="/trends" element={Trends} />
                        <Route exact path="/profile" element={Profile} />
                        <Route exact path="/gallery" element={Gallery} />
                    </Routes>
                </Router>
            </>
        )
    }
}
