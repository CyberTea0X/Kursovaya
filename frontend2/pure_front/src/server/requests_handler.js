import { userProfile, get_tags, get_many_tags, edit_tags, gallery, ip, port, get_avatar, get_image_data, all_user_profiles,
         get_user_chats, get_chat_messages, find_chat } from "./requests.js"
import { User, Image, Chat, Message } from "../types.js"


async function findChat(email, password, user2_id) {
    let data = await find_chat(email, password, user2_id);
    if (data.status !== "OK") {
        throw Error(data.reason);
    }
    return Chat.fromJson(data["chat"]);
}


async function getChatMessages(email, password, user2_id) {
    let data = await get_chat_messages(email, password, user2_id);
    if (data.status !== "OK") {
        throw Error(data.reason);
    }
    return data["messages"].map((message) => Message.fromJson(message));
}


async function getUserChats(email, password, include_user1=true, include_user2=true) {
    let data = await get_user_chats(email, password);
    if (data.status !== "OK") {
        throw Error(data.reason);
    }
    let chats = data["chats"].map((chat) => Chat.fromJson(chat));
    if (!include_user1 && !include_user2) {
        return;
    }
    let users_map = new Map( (await getAllUserProfiles()).map((user) => [user.id, user]));
    if (include_user1) {
        chats = chats.map((chat) => chat.withUser1(users_map.get(chat.userid1)))
    }
    if (include_user2) {
        chats = chats.map((chat) => chat.withUser2(users_map.get(chat.userid2)))
    }
    return chats;
}


async function getAvatarImage(user_id) {
    let data = await get_avatar(user_id)
    if (data.status !== "OK") {
        throw Error(data.reason);
    }
    data = await get_image_data(data["image_id"]);
    if (data.status !== "OK") {
        throw Error(data.reason);
    }
    return Image.fromJson(data["image"]).withUrl(ip, port);
}


async function getImages(user_id) {

    const data = await gallery(user_id);
    if (data.status !== "OK") {
      throw Error(data.reason);
    }
    const images = data["images"].map((image) => {
      image = Image.fromJson(image);
      image.setUrl(ip, port, user_id);
      return image;
    });
    return images;
}


function is_valid_tags(tags) {
    const regex = /^[a-zA-Z0-9]+(,[a-zA-Z0-9]+)*$/;
    // Регулярное выражение проверяет, что строка может содержать запятую и букву или цифру.
    // Это может повторяться неограниченное количество раз.
    const tagsArray = tags.split(",");
    const uniqueTags = new Set(tagsArray);
    return regex.test(tags) && tagsArray.length === uniqueTags.size;
}

async function getAllUserProfiles() {
    const data = await all_user_profiles();
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let users = data["users"].map((user) => {
        return User.fromJson(user)
    })
    return users;
}


async function getUserProfile(idOrEmail) {
    const data = await userProfile(idOrEmail);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let user = User.fromJson(data["user"]);
    if (isNaN(parseInt(idOrEmail))) {
        user.email = idOrEmail;
    }
    return user;
}

async function getManyTagsArray(range, add_hstag=true, sep=", ") {
    const data = await get_many_tags(range);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let items = data["items"]
    if (add_hstag) {
        items = items.map(tags => [tags[0], tags[1].map(tag => `#${tag}`).join(sep)]);
    } 
    return items;
}

async function getTagsArray(user_id, add_hstag=true) {
    const data = await get_tags(user_id);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let tags = data["tags"]
    if (tags === '') {
        return []
    }
    if (add_hstag) {
        tags = tags.map(tag => `#${tag}`);
    }
    return tags;
}

async function editTagsFromStr(email, password, newtags) {
    newtags = newtags.replace(/ /g, '').replace(/#/g, '')
    if (!is_valid_tags(newtags)) {
        throw Error("Invalid tags");
    }
    const data = await edit_tags(email, password, newtags);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
}

export {getUserProfile, getTagsArray, getManyTagsArray, editTagsFromStr, getImages, getAvatarImage, getAllUserProfiles, getUserChats,
        getChatMessages, findChat}