
import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import './search.css'
import { searchPopular } from "../../server/requests";
import { User as UserProfile} from "../../types";
import { Table } from './table'
import { getManyTagsArray } from '../../server/requests_handler';


const SearchPage = () => {
    const [query, setQuery] = useState("");
    const [users, setUsers] = useState([]);
    const [tags, setTags] = useState([]);

    const keys = ["first_name", "last_name", "email"]

      const search = async () => {
          let users_;
          await searchPopular().then(data => {
              // аутентификации
              users_ = data["items"]
          });
          users_ = users_.map(function(user) {
              return UserProfile.fromJson(user);
          });
          setUsers(users_);
          
      };
      const get_tags = async () => {
        let user_ids = users.map(user => user.id);
        let range = `${Math.min(...user_ids)}..${Math.max(...user_ids)}`
        setTags(await getManyTagsArray(range));

    };
    useEffect(() => {
        search();
    }, []); // вызываем search() только один раз при загрузке компонента

    useEffect(() => {
        if (users.length > 0) {
          get_tags();
        }
      }, [users]);
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
                <Table users={users} tags={tags}/>  
                
                
            </div>
           
        </div>
    )
}

export {SearchPage};


