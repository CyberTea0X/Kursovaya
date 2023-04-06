import config from '../config.json';


async function registerUser(login, email, password) {
    const { "backend-ip": ip, "backend-port": port } = config;
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

export { registerUser };