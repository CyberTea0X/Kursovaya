
import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import './search.css'
import { searchPopular, searchLogin, searchText, searchTags } from "../../server/requests";
import { User as UserProfile} from "../../types";
import { Table } from './table'
import { getManyTagsArray } from '../../server/requests_handler';


const SearchPage = () => {
    const [query, setQuery] = useState("");
    const [users, setUsers] = useState([]);
    const [tags, setTags] = useState([]);
    const [searchBy, setSearchBy] = useState("popular");

    const keys = ["first_name", "last_name", "email"]

      const search = async () => {
          let users_;
          switch (searchBy) {
            case "text":
                await searchText(query).then(data => {
                    users_ = data["items"]
                });
                break;
            case "login":
                await searchLogin(query).then(data => {
                    users_ = data["items"]
                });
                break;
            case "tags":
                let tags_ = query.replace(/#/g, '').replace(/ /g, '')
                await searchTags(tags_).then(data => {
                    users_ = data["items"]
                });
                break;
            case "popular":
                await searchPopular().then(data => {
                    users_ = data["items"]
                });
                break;
          }
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

    const handleSearchByChange = (event) => {
        setSearchBy(event.target.value);
    }
    
    const handleQueryChange = (event) => {
        setQuery(event.target.value);
        search();
    }
    return (
        
        <div className="searchpage">
            
                <div className='page-back'>
                  
                </div> 
                <div className='page'>
                <div className="search-bar">
                <input type="text" placeholder='Поиск...' className="search" onChange={handleQueryChange} />
                <select value={searchBy} onChange={handleSearchByChange}>
                    <option value="text">По тексту</option>
                    <option value="login">По логину</option>
                    <option value="tags">По тегам</option>
                </select>
                </div>
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


