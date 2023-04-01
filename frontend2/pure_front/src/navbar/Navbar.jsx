import './navbar.css';
import '../index.css';
import Logo from '../pages/img/751347.png';
import { Link } from 'react-router-dom';


function Header() {
  

  return (
    <div class='navbar'>

      <div class="leftside">
         <img src={Logo} style={{width: '60px', margin: '15px', float:'left'}} />  
         <p className='navbar-p'>DeltaArt</p>
      </div>

      <div class="rightside">
        <Link to="/search">
          <input className='searchbar' type="text" placeholder='Поиск...'  />
            
          
        </Link>
        <Link to="/">Главная</Link>
        <Link to="/trends">Популярное</Link>
        <Link to="/gallery">Gallery</Link>
        <Link to="/Account">Профиль</Link>
      </div>
    </div>
    )
   }


   export {Header}