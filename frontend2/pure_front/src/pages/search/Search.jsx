
import { useState } from 'react'
import { Link } from 'react-router-dom'
import './search.css'
import { Users } from "./users"



const Table = ({ data }) => {
    return (
      <table>
        <tbody>
          <tr>
            {/* <th>Profile</th>
            <th>Name</th>
            <th>Surname</th> */}
          </tr>
          {data.map((item) => (
            <tr key={item.id}>
              <td>{<img src="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQkwNi3BaNAgeNdiKbvkcaQGEa8ZgVBHK5dmOxaKrc&s" style={{
                width: '70px',
                height: '70px',
                borderRadius: '50%',
                border: '2px solid gray'
              }} />}</td>
              
              
              <td>{item.first_name}</td>
            </tr>
          ))}
        </tbody>
      </table>
    );
  };


const SearchPage = () => {
    const [query, setQuery] = useState("");

    const keys = ["first_name", "last_name", "email"]

    const search = (data) => {
        return data.filter((item) => 
        keys.some(key=>item[key].toLowerCase().includes(query.toLowerCase()))

        );
    };


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
                <Table data={search(Users)}/>  
                
                
            </div>
           
        </div>
    )
}

export {SearchPage};


