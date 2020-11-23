import axios from "axios";
import jwt from "jsonwebtoken";

const client = axios.create({
  baseURL: window.location.origin + '/',
  timeout: 1000,
});

function get_gestures(page, search_text) {
  let uri = 'gestures?max=10&page=' + page;
    if (search_text) {
      uri += '&search=' + search_text
    }
    return client
      .get(uri)
      .then(res =>  {
        let total_pages;
        
          total_pages = parseInt(res.headers['total-items'], 10);
        if (isNaN(total_pages)){
          total_pages = 1;
        }
        return { gestures: res.data, total_pages  }
      });
}

function delete_gesture(id) {
  return client.delete('gestures/' + id).then(() => undefined)
}

function delete_description(id) {
  return client.delete('descriptions/' + id).then(() => undefined)
}

function delete_meaning(id) {
  return client.delete('meanings/' + id).then(() => undefined)
}

function delete_picture(id) {
  return client.delete('pictures/' + id).then(() => undefined)
}

function put_description(id, new_description) {
  return client.put('descriptions/' + id,  new_description).then(() => undefined)
}

function put_meaning(id, new_meaning) {
  return client.put('meanings/' + id,  new_meaning).then(() => undefined)
}

function put_gesture(id, new_gesture) {
  return client.put('gestures/' + id,  new_gesture).then(() => undefined)
}

function put_picture_meta(id, new_picture_meta) {
  return client.put('pictures/' + id + '/meta',  new_picture_meta).then(() => undefined)
}

function put_picture_file(id, new_picture_file) {
  let formData = new FormData();
  formData.append('picture', new_picture_file);

  return client.put('pictures/' + id + '/file',  formData, { headers: { 'Content-Type': 'multipart/form-data' } }).then(() => undefined)
}

function post_description_meaning(id_description, new_meaning) {
  return client.post('descriptions/' + id_description + '/meanings',  new_meaning).then(() => undefined)
}

function post_gesture_meaning(id_gesture, new_meaning) {
  return client.post('gestures/' + id_gesture + '/meanings',  new_meaning).then(() => undefined)
}


function post_description(id_gesture, new_description) {
  return client.post('gestures/' + id_gesture + '/descriptions',  new_description).then(() => undefined)
}

function post_gesture(gesture) {
  return client.post('gestures', gesture).then((res) => res.data )
}

function post_picture(id_gesture, langs, file) {
  let formData = new FormData();
  formData.append('picture', file);

  return client.post('gestures/'+id_gesture +'/pictures?langs='  + langs.join(';'),  formData, { headers: { 'Content-Type': 'multipart/form-data' } }).then(() => undefined)
}

function login(credentials) {
  return client.post('login',  credentials).then((res) => {
    sessionStorage.setItem('jwt', res.data);
    return jwt.decode(res.data);
  })
}

client.interceptors.request.use(
  config => {
    const token = sessionStorage.getItem('jwt');

    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    } else {
      delete client.defaults.headers.common.Authorization;
    }
    return config;
  },

  error => Promise.reject(error)
);

export const service = {
  get_gestures,
  delete_gesture,
  delete_description,
  delete_meaning,
  delete_picture,
  put_description,
  put_meaning,
  put_picture_meta,
  put_gesture,
  put_picture_file,
  post_description_meaning,
  post_gesture_meaning,
  post_description,
  post_gesture,
  post_picture,
  login,
}