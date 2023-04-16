import { useState, useEffect } from 'react'

const Table = ({ users, tags }) => {
  // создаем хэш-таблицу для тегов
  const [tagsMap, setTagsMap] = useState(new Map());

  useEffect(() => {
    if (tags.length > 0) {
      setTagsMap(new Map(tags.map(([id, tags]) => [id, tags])));
    }
  }, [tags]);
    return (
      <table>
        <tbody>
          <tr>
            
            {/* <th>Profile</th>
            <th>Name</th>
            <th>Surname</th> */}
          </tr>
          {users.map((item) => (
            <tr key={item.id}>
              <td>{<img src="https://avatars.dzeninfra.ru/get-zen_doc/1884623/pub_60be1f9abcbf42494ea7da85_60be206a746af706906e32df/scale_1200" style={{
                width: '50px',
                height: '50px',
                borderRadius: '25px',
              }} />}{item.first_name}</td>
              <td>{item.username}</td>
              <td>{tagsMap.get(item.id)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    );
  };

export {Table}