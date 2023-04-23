import './navbar.css';
import '../index.css';
import Logo from '../img/751347.png';
import { Link } from 'react-router-dom';
import Search from './Search2.png';
import Chat from './chat2.png'



function Header() {


  return (
    <div className='navbar'>

      <div className="leftside">
         <img alt="" src={Logo} style={{width: '60px', margin: '15px', float:'left'}} />  
         <p className='navbar-p'>DeltaArt</p>
      </div>

      <div className="rightside">
        

        <Link to="/">Главная</Link>
        <Link to="/Search"><img alt="" src={Search} style={{width: '45px', margin: '15px', float:'left'}} /></Link>
        <Link to="/Trends">Популярное</Link> 
        <Link to="/Messenger"><img alt="" src={Chat} style={{width: '40px', margin: '15px', float:'left'}} /></Link>       
        <Link to="/Profile/me">Профиль</Link>
      </div>
    </div>
    )
   }


   export {Header}
   
   
   