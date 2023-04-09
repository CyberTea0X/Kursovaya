
import { useState } from 'react'
import { Link } from 'react-router-dom'
import './search.css'
import { searchPopular } from "../../server/server";
import { User as UserProfile} from "../../types";
import { Table } from './table'


const SearchPage = () => {
    const [query, setQuery] = useState("");
    const [users, setUsers] = useState([]);

    const keys = ["first_name", "last_name", "email"]

    const search = async () => {
      let users;
      await searchPopular().then(data => {
        // аутентификации
        users = data["items"]
        
      });
      users = users.map(function(user) {
        return UserProfile.fromJson(user);
      });
      console.log(users);
      setUsers(users);
    };

    search();
    return (
        
        <div className="searchpage">
            
                <div className='page-back'>
                  
                </div> 
                <div className='page'>
                <input type="text" placeholder='Поиск...' className="search" onChange={e=> setQuery(e.target.value)} />
                <p className='p'>Вы можете найти художника не только по имени, но и по тегам:</p>
                <p className='p2'> #Traditional<br/>
                        #Digital<br/>
                        #Portraits<br/>
                        #Animalistic<br/>
                        #Anime<br/>
                        #Nature<br/>
                        #Landscape<br/></p>
                <Table data={users}/>  
                
                
            </div>
           
        </div>
    )
}

export {SearchPage};


