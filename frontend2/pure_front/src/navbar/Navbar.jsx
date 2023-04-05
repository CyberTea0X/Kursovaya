import './navbar.css';
import '../index.css';
import Logo from '../pages/img/751347.png';
import { Link } from 'react-router-dom';
import Search from './Search2.png';
import Chat from './chat2.png'



function Header() {


  return (
    <div class='navbar'>

      <div class="leftside">
         <img src={Logo} style={{width: '60px', margin: '15px', float:'left'}} />  
         <p className='navbar-p'>DeltaArt</p>
      </div>

      <div class="rightside">
        

        <Link to="/">Главная</Link>
        <Link to="/Search"><img src={Search} style={{width: '45px', margin: '15px', float:'left'}} /></Link>
        <Link to="/Trends">Популярное</Link> 
        <Link to="/Chat"><img src={Chat} style={{width: '40px', margin: '15px', float:'left'}} /></Link>       
        <Link to="/Account">Профиль</Link>
      </div>
    </div>
    )
   }


   export {Header}
   
   
   