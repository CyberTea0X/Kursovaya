import config from '../config.json';


const { "backend-ip": ip, "backend-port": port } = config;

async function registerUser(login, email, password) {
    const url = `http://${ip}:${port}/api/user/register/${login}/${email}/${password}`;
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


async function userProfile(idOrEmail) {
    const query = (typeof idOrEmail === 'number') ? `id={idOrEmail}`: `email={idOrEmail}`;
    const url = `http://${ip}:${port}/api/user/profile?${query}`;
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


async function login(email, password) {
    const url = `http://${ip}:${port}/api/user/login/${email}/${password}`;
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


export { registerUser, login, userProfile };