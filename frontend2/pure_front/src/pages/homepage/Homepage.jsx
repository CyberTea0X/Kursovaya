import './home.css';
import { Link } from 'react-router-dom';
import React from 'react';
import Fox from './лиса.jpg';
import Tea from './tea.png';
import Gosling from './гослинг.png';
import Cat from './header5.jpg';
import CloudDog from './header2.jpg';
import Corgi from './header3.jpg';
import GoodBoy from './header4.jpg';
import Chat from './chat.png';
import Profile from './profile.png';
import Popular from './popular.png';




const Homepage = () => {
  return (
    <div >
      <div className='heading'>
        <h1 className='home-title2'>Добро пожаловать на платформу <br /> от художников для художников</h1>
        
          <img className='heading-pics' src={Cat} />
          <img className='heading-pics2' src={CloudDog} />
          <img className='heading-pics3' src={Corgi} />
          <img className='heading-pics4' src={GoodBoy} />
        
        

      </div>

      <div className='about-us'>

        <h1 className='home-title'>Местный Райан Гослинг <br /> встретит всех с <br /> чашечкой виртуального чая <br /> и лисичками</h1>
        <p className='text'>Для наибольшего комфорта <br /> пользователей <br /> <Link to="https://vk.com/tolya77m">пишите</Link> свои предложения <br /> по улучшению сайта <br/> или внедрению новых фишек.</p>
        <img className='home-tea' src={Tea} />
        <img className='home-gosling' src={Gosling} />        
        <img className='home-fox' src={Fox} />

      </div>

      <div className='functional'>
        <h1 className='home-title2'>Используйте функционал по полной!</h1>
        <Link  to="/trends"><img className='func-img1' src={Popular}/></Link>
        <Link><img className='func-img2' src={Chat}/></Link>
        <Link to="/trends"><img className='func-img3' src={Profile}/></Link>
        <p className='text2' style={{	left: '310px', top:'75px'}}>Загляните на вкладку "Популярное" и найдите актуального для себя художника</p>
        <p className='text2' style={{	left: '310px', top:'270px'}}>Общайтесь со своими заказчиками или с теми, у кого заказали картину</p>
        <p className='text2' style={{	left: '310px', top:'444px'}}>Настраивайте свой профиль, выбирайте теги,<br/> под которыми вы рисуете, и Вас обязательно заметят</p>
        


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
