import config from '../config_local.json';


const { "backend-ip": ip, "backend-port": port } = config;



async function send_message(email, password, userId2, message) {
    const query = `?content=${message}`;
    const url = `http://${ip}:${port}/api/messages/send/${email}/${password}/${userId2}${query}`;
    return postRequest(url);
}

async function read_all_chat_messages(email, password, user2_id) {
    const url = `http://${ip}:${port}/api/messages/readall/${email}/${password}/${user2_id}`;
    return postRequest(url);
}

async function read_chat_messages(email, password, user2_id, messages) {
    const xml = new XMLHttpRequest();
    const url = `http://${ip}:${port}/api/messages/read/${email}/${password}/${user2_id}`;
    const options = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(messages)
      };
    const response = await fetch(url, options);

    return response.json;
}

async function get_chat_messages(email, password, user2_id) {
    const url = `http://${ip}:${port}/api/messages/get/${email}/${password}/${user2_id}`
    return postRequest(url);
}

async function create_chat(email, password, user2id) {
    const url = `http://${ip}:${port}/api/chat/create/${email}/${password}/${user2id}`
    return postRequest(url);
}

async function get_user_chats(email, password) {
    const url = `http://${ip}:${port}/api/chat/user/${email}/${password}`
    return postRequest(url);
}

async function all_user_profiles() {
    const url = `http://${ip}:${port}/api/user/profiles`
    return postRequest(url);
}

async function delete_profile(email, password) {
    const url = `http://${ip}:${port}/api/user/delete/${email}/${password}`
    return postRequest(url);
}

async function get_avatar(user_id) {
    const url = `http://${ip}:${port}/api/logo/get/${user_id}`;
    return getRequest(url);
}

async function set_as_avatar(email, password, image_id) {
    const url = `http://${ip}:${port}/api/logo/set/${email}/${password}/${image_id}`;
    return postRequest(url);
}

async function visit(email, password, id) {
    const url = `http://${ip}:${port}/api/user/visit/${email}/${password}/${id}`;
    return postRequest(url);
}

async function edit_image_data(email, password, image_id, about="", image_name="", tags="") {
    let query = []
    if (about) query.push(`about=${about}`);
    if (image_name) query.push(`image_name=${image_name}`);
    if (tags) query.push(`tags=${tags}`);
    query = (query) ? '?' + query.join("&"): "";
    //const query = `about=${about}&image_name=${image_name}&tags=${tags}`
    const url = `http://${ip}:${port}/api/images/data/edit/${email}/${password}/${image_id}${query}`;
    return postRequest(url)
}

async function get_image_data(image_id) {
    const url = `http://${ip}:${port}/api/images/data/get/${image_id}`;
    return getRequest(url)
}

async function change_image(file, email, password, image_id) {
    const formData = new FormData();
    formData.append("file", file);
  
    const url = `http://${ip}:${port}/api/images/change/${email}/${password}/${image_id}`;
  
    const response = await fetch(url, {
      method: "POST",
      body: formData
    });
  
    const data = await response.json();
    return data;
}

async function upload_image(email, password, file, about="", image_name="", tags="") {
  about = about || "Автор ничего не рассказал о картинке";
  image_name = image_name || "Без имени";
  tags = tags || "new";
  const query = `about=${about}&image_name=${image_name}&tags=${tags}`
  const formData = new FormData();
  formData.append("file", file);

  const url = `http://${ip}:${port}/api/images/load/${email}/${password}?${query}`;

  const response = await fetch(url, {
    method: "POST",
    body: formData
  });

  const data = await response.json();
  return data;
}

async function delete_image(email, password, image_id) {
    const url = `http://${ip}:${port}/api/images/delete/${email}/${password}/${image_id}`;
    return postRequest(url)
}

async function gallery(user_id) {
    const url = `http://${ip}:${port}/api/gallery/${user_id}`;
    return getRequest(url)
}

async function edit_tags(email, password, new_tags) {
    const url = `http://${ip}:${port}/api/user/tags/edit/${email}/${password}?tags=${new_tags}`;
    return postRequest(url)
}

async function get_many_tags(range) {
    const url = `http://${ip}:${port}/api/user/tags/many/${range}`;
    return postRequest(url)
}

async function get_tags(user_id) {
    const url = `http://${ip}:${port}/api/user/tags/one/${user_id}`;
    return postRequest(url)
}

async function searchTags(tags) {
    const url = `http://${ip}:${port}/api/search/tags/${tags}`;
    return postRequest(url)
}

async function searchText(text) {
    const url = `http://${ip}:${port}/api/search/text/${text}`;
    return postRequest(url)
}


async function searchLogin(login) {
    const url = `http://${ip}:${port}/api/search/login/${login}`;
    return postRequest(url)
}


async function searchPopular() {
    const url = `http://${ip}:${port}/api/search/popular`;
    return postRequest(url)
}


async function registerUser(login, email, password) {
    const url = `http://${ip}:${port}/api/user/register/${login}/${email}/${password}`;
    return postRequest(url)
}


async function editUser(email, password, user) {
    const query = user.toQuery()
    const url = `http://${ip}:${port}/api/user/edit/${email}/${password}?${query}`;
    return postRequest(url)
}


async function userProfile(idOrEmail) {
    const query = (isNaN(parseInt(idOrEmail))) ? `email=${idOrEmail}`: `id=${idOrEmail}`;
    const url = `http://${ip}:${port}/api/user/profile?${query}`;
    return postRequest(url)
}


async function login(email, password) {
    const url = `http://${ip}:${port}/api/user/login/${email}/${password}`;
    return postRequest(url)
}

async function postRequest(url) {
    return new Promise((resolve, reject) => {
        fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
        })
        .then(response => response.json())
        .then(data => resolve(data))
        .catch(error => reject(error));
    });
}

async function getRequest(url) {
    try {
        const response = await fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json'
            },
        });
        const data = await response.json();
        return data;
    } catch (error) {
        throw new Error(error);
    }
}


export { ip, port, registerUser, login, userProfile, editUser, searchLogin, searchPopular, searchText, get_tags, edit_tags,
         get_many_tags, searchTags, change_image, upload_image, get_image_data, edit_image_data, gallery, delete_image,
         visit, set_as_avatar, get_avatar, delete_profile, all_user_profiles, get_user_chats, create_chat, get_chat_messages,
         read_all_chat_messages, read_chat_messages, send_message};