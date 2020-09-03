import axios from "axios";

const client = axios.create({
    baseURL: 'http://localhost:8000',

});

function login(username, password) {
    return client.post('/login', { username, password })
        .then((res) => {
            console.log(res);
            client.defaults.headers.common['Authorization'] = 'Bearer ' + res.data;
        })
        .catch((e) => console.log("Fail to login", e));
}

function gestures() {
    return client.get('/gestures').catch((e) => console.log("Fail to retrieve gestures", e));
}

function add_gesture(gesture) {
    return client.post('/gestures', gesture).catch((e) => console.log("Fail to create gesture", e));
}

function add_description(id_gesture, description) {
    return client.post('/gestures/' + id_gesture, description).catch((e) => console.log("Fail to create description", e));
}

function add_description_s_meaning(id_description, meaning) {
    return client.post('/descriptions/' + id_description, meaning).catch((e) => console.log("Fail to create meaning", e));
}

function add_gesture_s_meaning(id_gesture, meaning) {
    return client.post('/gestures/' + id_gesture, meaning).catch((e) => console.log("Fail to create meaning", e));
}

function add_picture(id_gesture, picture) {
    return client.post('/gestures/' + id_gesture, picture).catch((e) => console.log("Fail to create picture", e));
}

function delete_gesture(id_gesture) {
    return client.delete('/gestures/' + id_gesture).catch((e) => console.log("Fail to delete gesture", e));
}

function delete_description(id_description) {
    return client.delete('/descriptions/' + id_description).catch((e) => console.log("Fail to delete description", e));
}

function delete_meaning(id_meaning) {
    return client.delete('/meanings/' + id_meaning).catch((e) => console.log("Fail to delete meaning", e));
}

function delete_picture(id_picture) {
    return client.delete('/pictures/' + id_picture).catch((e) => console.log("Fail to de;ete picture", e));
}

export { login, gestures, add_gesture, add_description, add_gesture_s_meaning, add_description_s_meaning, add_picture, delete_gesture, delete_description, delete_meaning, delete_picture };