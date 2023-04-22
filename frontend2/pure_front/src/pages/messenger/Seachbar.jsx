import React from 'react';
import "./styles/searchbar.css";

const Searchbar = ({onSearch}) => {

    return (
        <div className='search-container'>
          <input type="text" onChange={onSearch} className='search-input' />
        </div>
      );
};

export { Searchbar };