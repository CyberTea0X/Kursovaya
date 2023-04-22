import React, { useState, useEffect } from 'react';
import "./styles/Searchbar.css";

const Searchbar = ({chats, users}) => {

    const [searchText, setSearchText] = useState('');
    const [items, setItems] = useState([]);
    
    const handleSearch = (event) => {
      setSearchText(event.target.value);
    };

    useEffect(() => {
      if (chats === undefined) {
        return;
      }
      console.log(chats);
      const new_items = chats.filter((item) =>
        item.created_at.toLowerCase().includes(searchText.toLowerCase())
      );
      setItems(new_items);
        

    }, [chats]);

    return (
        <div className='search-container'>
          <input type="text" value={searchText} onChange={handleSearch} className='search-input' />
          <ul>
            {items.map((item) => (
              <li key={item.id}> Amogus </li>
            ))}
          </ul>
        </div>
      );
};

export { Searchbar };