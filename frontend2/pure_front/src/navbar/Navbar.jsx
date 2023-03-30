import './navbar.css'
import '../index.css'
import Logo from '../pages/img/751347.png'
import { Link } from 'react-router-dom';


function Header() {
    return (
     <div class='navbar'>
       <div class="leftside">
         
         <img src={Logo} style={{width: '70px', margin: '15px', float:'left'}} />  
         <p>DeltaArt</p>
       </div>
       <div class="rightside">
           <Link to="/">Home</Link>
           <Link to="/trends">Trends</Link>
           <Link to="/gallery">Gallery</Link>
           <Link to="/profile">Profile</Link>
       </div>
     </div>
    )
   }
   export {Header}