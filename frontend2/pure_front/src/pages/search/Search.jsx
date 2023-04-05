
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
              <td>{<img src="https://avatars.dzeninfra.ru/get-zen_doc/1884623/pub_60be1f9abcbf42494ea7da85_60be206a746af706906e32df/scale_1200" style={{
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
                <Table data={search(Users)}/>  
                
                
            </div>
           
        </div>
    )
}

export {SearchPage};


