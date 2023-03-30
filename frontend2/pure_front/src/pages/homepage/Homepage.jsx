import './home.css'
import { Link } from 'react-router-dom'
import Pic1 from './Pic1.png'
import React from 'react'

const Homepage = () => {
  return (
    <div>

      <div style={{backgroundImage: "url(https://catherineasquithgallery.com/uploads/posts/2021-02/1612305975_5-p-fon-purpurnii-gradient-5.jpg)"}}>
        <Link to="/Profile">
          <button>Зарегистрироваться</button>
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
