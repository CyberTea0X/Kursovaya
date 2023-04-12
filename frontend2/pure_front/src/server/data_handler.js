import userProfile from "/requests.js"

async function getUserProfile(email) {
    const data = await userProfile(email);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let user = UserProfile.fromJson(data["user"]);
    user.email = email;
    return user;
}