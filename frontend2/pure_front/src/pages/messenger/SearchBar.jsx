import React, { useState } from 'react';
import './styles/SearchBar.css';

const SearchBar = () => {
  const [search, setSearch] = useState('');

  const handleChange = (event) => {
    setSearch(event.target.value);
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    // обработчик поиска
  };

  return (
    <form className="search-bar" onSubmit={handleSubmit}>
      <input type="text" value={search} onChange={handleChange} />
      <button type="submit">Search</button>
    </form>
  );
};

export { SearchBar };