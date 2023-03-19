import React, { Component } from 'react'
import { Container, NavLink, TabContainer, Row, Col, Nav, Tab } from 'react-bootstrap'

export default class Contacts extends Component {
  render() {
    return (
      <Container> 
        <TabContainer id = "ledt-tabs-example" defaultActiveKey="first">
          <Row>
            <Col sm={3} >
              <Nav variant="pills" classname= "flex-column mt-2">

                <Nav.Item>
                  <NavLink eventKey="first"> Lead </NavLink>
                </Nav.Item>

                <Nav.Item>
                  <NavLink eventKey="second"> Back </NavLink>
                </Nav.Item>

                <Nav.Item>
                  <NavLink eventKey="third"> Front </NavLink>
                </Nav.Item>

                <Nav.Item>
                  <NavLink eventKey="fourth"> Front2 </NavLink>
                </Nav.Item>

                <Nav.Item>
                  <NavLink eventKey="fifth"> Front3 </NavLink>
                </Nav.Item>

              </Nav>
            </Col> 
            <Col sm={9}>
              <Tab.Content>
                <Tab.Pane eventKey= "first">
                  <img src='https://img1.ak.crunchyroll.com/i/spire1/c649d167e5e9e7db84dc3fc5561072411667481247_main.jpg'/>
                  <p>
                    Анатолий Михайлович Михайлусов
                  </p>
                </Tab.Pane>
                <Tab.Pane eventKey= "second">
                  <img src='https://images.nintendolife.com/451cf894dcdd2/bayonetta.900x.jpg'/>
                  <p>
                    Федор Лоскутов
                  </p>
                </Tab.Pane>
                <Tab.Pane eventKey= "third">
                  <img src='https://static.wikia.nocookie.net/chainsaw-man/images/5/5a/Power_anime.png/revision/latest?cb=20221109160327&path-prefix=es'/>
                  <p>
                    Даниил Дмитриевич
                  </p>
                </Tab.Pane>
                <Tab.Pane eventKey= "fourth">
                  <img src='https://static.wikia.nocookie.net/chainsaw-man/images/5/5a/Power_anime.png/revision/latest?cb=20221109160327&path-prefix=es'/>
                  <p>
                    Пушенко Артем Сергеевич
                  </p>
                </Tab.Pane>
                <Tab.Pane eventKey= "fith">
                  <img src='https://static.wikia.nocookie.net/chainsaw-man/images/5/5a/Power_anime.png/revision/latest?cb=20221109160327&path-prefix=es'/>
                  <p>
                    Новрузова Дарья Гейсовна
                  </p>
                </Tab.Pane>
              </Tab.Content>
            </Col>
          </Row>
        </TabContainer>
      </Container>
    )
  }
}
