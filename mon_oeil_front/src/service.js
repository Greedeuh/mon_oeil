import axios from "axios";

const client = axios.create({
    baseURL: 'http://localhost:8000/',
    timeout: 1000,
    headers: {'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjIyMDM0MDcxOTgsImxldmVsIjoiQWRtaW4ifQ.RLE2du-ICZ0mlFl02YytZC02Xk0U5qyNRBxhi_-SvY8',
    'Access-Control-Allow-Origin': '*'},

  });


function get_all_gestures() {
   return client.get('gestures').then(res => res.data)
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

export const service = {
    get_all_gestures,
    delete_gesture,
    delete_description,
    delete_meaning,
    delete_picture,
    put_description,
    put_meaning,
    put_picture_meta,
    put_picture_file,
    post_description_meaning,
    post_gesture_meaning,
    post_description    
}