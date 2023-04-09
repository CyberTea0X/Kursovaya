import config from '../config.json';


const { "backend-ip": ip, "backend-port": port } = config;


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


export { registerUser, login, userProfile, editUser, searchLogin, searchPopular, searchText };