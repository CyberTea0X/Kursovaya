import config from '../config_local.json';


const { "backend-ip": ip, "backend-port": port } = config;


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
    console.log(user)
    const query = user.toQuery()
    const url = `http://${ip}:${port}/api/user/edit/${email}/${password}?${query}`;
    return postRequest(url)
}


async function userProfile(idOrEmail) {
    const query = (typeof idOrEmail === 'number') ? `id=${idOrEmail}`: `email=${idOrEmail}`;
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


export { registerUser, login, userProfile, editUser, searchLogin, searchPopular, searchText, get_tags, edit_tags,
         get_many_tags, searchTags };