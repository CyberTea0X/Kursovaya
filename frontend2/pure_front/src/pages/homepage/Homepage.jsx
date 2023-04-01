import './home.css'
import { Link } from 'react-router-dom'
import React from 'react'

const Homepage = () => {
  return (
    <div >
      <div className='heading'>
      </div>

      <div className='about-us'>
        <p className='text'>Hello everyone! Let’s edit some text.</p>
      </div>

      <div className='functional'>
        <Link to="/Signup">
          <button className='registration-button'>Зарегистрироваться</button>
        </Link>

        <Link to="/Login">
          <button className='enter-button'>Войти</button>       
        </Link>
      </div>

    </div>
  )
}

export { Homepage }







// import React, { Component } from 'react'

// class Homepage extends Component {
//     render() {
//     return (                
           
//             <div>               
//                 <section className='hero'>
//                     <div className='hero-body'>
//                         <div className='container'>
//                             <h1 className='title'>{'Добро пожаловать!'}</h1>
//                             <div className='is-two-thirds column is-paddingless'>
//                                 <h2 className='subtitle is-4'>{'Лучший сервис для художника'}</h2>                                
//                             </div>                            
                           
//                         </div>
//                     </div>
//                 </section>


//                 <section className='section'>
//                 <div className='container'>
//                     <div className='columns pd is-desktop'>
//                         <div className='column is-1 has-text-centered'>
//                             <i className='fa fa-cog is-primary'></i>
//                         </div>
//                         <div className='columns is-one-third-desktop'>
//                             <p className='title'><strong>Функционал сайта</strong></p>
//                         </div>                       
//                     </div>
               


               
//                     <div className='column'>
//                         <div className='card'>
//                             <div className='card-content'>
//                                 <p className='title'>Информация о сайте</p>
//                                 <p className='subtitle'>- Доп</p>
//                             </div>
//                         </div>
//                     </div>                    
               

//                     <div className='column'>
//                         <div className='card'>
//                             <div className='card-content'>
//                                 <p className='title'>Информация о сайте</p>
//                                 <p className='subtitle'>- Доп</p>
//                             </div>
//                         </div>
//                     </div>                    
              

              
//                     <div className='column'>
//                         <div className='card'>
//                             <div className='card-content'>
//                                 <p className='title'>Информация о сайте</p>
//                                 <p className='subtitle'>- Доп</p>
//                             </div>
//                         </div>
//                     </div>                    
//                 </div>
               
//                 </section>
//                 <h2 style={{textAlign: 'center'}}><Link to="/profile">Зарегестрироваться/Войти</Link></h2>     
//             </div> 
           
//     )  
// }
// }

// export default Homepage;
