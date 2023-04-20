import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom';
import { getAvatarImage } from '../../server/requests_handler';
import UnknownPerson from "../img/Unknown_person.jpg";

const Table = ({ users, tags }) => {
  const [tagsMap, setTagsMap] = useState(new Map());
  const [avatars, setAvatars] = useState({});

  let navigate = useNavigate(); 
  const goToProfile = (id) =>{ 
    let path = `/Profile/${id}`; 
    navigate(path);
  }

  const loadAvatar = async (user_id) => {
    try {
      let logo_img = await getAvatarImage(user_id);
      return logo_img.url;
    }
    catch (exc) {
      return '';
    }
  }

  const loadAvatars = async (users) => {
    let new_avatars = {}
    await Promise.all(users.map(async (user) => {
      let avatar = await loadAvatar(user.id);
      new_avatars[user.id] = avatar;
    }))
    setAvatars(new_avatars);
  }

  useEffect(() => {
    if (tags.length > 0) {
      setTagsMap(new Map(tags.map(([id, tags]) => [id, tags])));
    }
  }, [tags]);

  useEffect(() => {
    if (users !== undefined) {
      loadAvatars(users);
    }
  }, [users]);

  return (
    <table>
      <tbody>
        <tr>
          {/* <th>Profile</th>
          <th>Name</th>
          <th>Surname</th> */}
        </tr>
        {users.map((item) => (
          <tr key={item.id} onClick={() => goToProfile(item.id)}>
            <td>
              <img 
                src={avatars[item.id] || UnknownPerson} 
                style={{
                  width: '50px',
                  height: '50px',
                  borderRadius: '25px',
                  objectFit: 'cover'
                }}
              />
              {item.first_name}
            </td>
            <td>{item.username}</td>
            <td>{tagsMap.get(item.id)}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export {Table}