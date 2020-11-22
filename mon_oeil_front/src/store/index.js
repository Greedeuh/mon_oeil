import Vue from "vue";
import Vuex from "vuex";

import { service } from '../service';

Vue.use(Vuex);

let mock_gestures = [
  {
    id: "Bien!1",
    descriptions: [
      {
        id: "Bien!1",
        value: "Le pouce est vers le haut",
        langs: ["fr", "us"],
        meanings: [
          {
            value: "Super!, Bien joué!, Nickel ou simplement Bien.",
            langs: ["fr", "us"],
          },
          {
            value: "Bien joué!, Nickel ou simplement Bien.",
          },
        ],
      },
      {
        value: "Le pouce est vers le bas",
        langs: ["fr", "us"],
        meanings: [
          {
            value: "C'est perdu!, Pas cool... c'est mort.",
          },
        ],
      },
    ],

    meanings: [
      {
        value: " Super!, Bien joué!, Nickel ou simplement Bien.",
        langs: ["us"],
      },
    ],
    pictures: [{ id: "xqxq", langs: ["fr", "us"] }],
  },
  {
    id: "Bien!",
    descriptions: [
      {
        value: "Le pouce est vers le haut",
        langs: ["fr", "us"],
        meanings: [
          {
            value: "Super!, Bien joué!, Nickel ou simplement Bien.",
            langs: ["fr", "us"],
          },
          {
            value: "Bien joué!, Nickel ou simplement Bien.",
          },
        ],
      },
      {
        value: "Le pouce est vers le bas",
        langs: ["fr", "us"],
        meanings: [
          {
            value: "C'est perdu!, Pas cool... c'est mort.",
          },
        ],
      },
    ],

    meanings: [
      {
        value: " Super!, Bien joué!, Nickel ou simplement Bien.",
        langs: ["us"],
      },
    ],
    pictures: [{ id: "xqxq", langs: ["fr", "us"] }],
  },
];

const notif_ok = { msg: "Action effectuée!", success: true };
const notif_ko = { msg: "Oups ça n'a pas fonctionné :( Reviens plus tard", success: false };
const notif_ko_login = { msg: "Les identifiants ne sont pas correct :(", success: false };

export default new Vuex.Store({
  state: {
    search_count: 0,
    gestures: mock_gestures,
    editor_mode: false,
    selected_id_gesture: null,
    loading: false,
    notif: null,
    search: {
      page: 1,
      total_pages:1,
      text: ""
    },
    user: {
      logging_in: false,
      jwt_payload: null
    }
  },
  getters: {
    gestures(state) {
      return state.gestures;
    },
    selected_gesture(state) {
      return state.gestures.find(
        (gesture) => gesture.id === state.selected_id_gesture
      );
    },
    editor_mode(state) {
      return state.editor_mode;
    },
    loading(state) {
      return state.loading;
    },
    notif(state) {
      return state.loading;
    },
    search(state) {
      return state.search;
    },
    user(state) {
      return state.user;
    }
  },
  mutations: {
    toggle_editor_mode(state) {
      state.editor_mode = !state.editor_mode;
    },
    select_gesture(state, selected_id_gesture) {
      state.selected_id_gesture = selected_id_gesture;
    },
    start_loading(state) {
      state.loading = true;
    },
    stop_loading(state) {
      state.loading = false;
    },
    load_gestures(state, {gestures, total_pages}) {
        console.log(total_pages)
      state.search_count++;
      state.search = { ...state.search, total_pages: Math.ceil(total_pages / 10)};
      state.gestures = gestures;
    },
    notif(state, notif) {
      state.notif = notif;
    },
    search_text(state, search_text) {
      state.search.text = search_text;
    },
    go_page(state, num) {
      state.search.page = num;
    },
    logging_in(state, yes_no) {
      state.user.logging_in = yes_no;
    },
    jwt_payload(state, jwt_payload) {
      state.user.jwt_payload = jwt_payload;
    }
  },
  actions: {
    load_gestures(context) {
      let search = context.state.search;
      service.get_gestures(search.page, search.text).then(({gestures, total_pages}) => {
        context.commit("load_gestures", { gestures, total_pages })
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko)
      })
      context.commit("stop_loading");
    },
    del_gesture(context, id) {
      context.commit("start_loading");
      service.delete_gesture(id).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    del_description(context, id) {
      context.commit("start_loading");
      service.delete_description(id).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    del_meaning(context, id) {
      context.commit("start_loading");
      service.delete_meaning(id).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    del_picture(context, id) {
      context.commit("start_loading");
      service.delete_picture(id).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    update_description(context, { id, new_description }) {
      context.commit("start_loading"); 
      service.put_description(id, new_description).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    update_meaning(context, { id, new_meaning }) {
      service.put_meaning(id, new_meaning).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    update_picture_meta(context, { id, new_picture_meta }) {
      service.put_picture_meta(id, new_picture_meta).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    update_picture_file(context, { id, new_picture_file }) {
      service.put_picture_file(id, new_picture_file).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    add_description_meaning(context, {id_description, new_meaning}) {
      context.commit("start_loading"); 
      service.post_description_meaning(id_description, new_meaning).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    add_gesture_meaning(context, {id_gesture, new_meaning}) {
      context.commit("start_loading"); 
      service.post_gesture_meaning(id_gesture, new_meaning).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    add_description(context, {id_gesture, new_description}) {
      context.commit("start_loading"); 
      service.post_description(id_gesture, new_description).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    add_gesture(context, new_gesture) {
      service.post_gesture(new_gesture).then((new_id_gesture) => {
        context.dispatch('load_gestures');
        context.commit('select_gesture', new_id_gesture)
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    add_picture(context, { id_gesture, langs, file }) {
      service.post_picture(id_gesture, langs, file).then(() => {
        context.dispatch('load_gestures');
        context.commit('notif', notif_ok);
      }).catch((e) => {
        console.error(e);
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    search_text(context, search_text) {
      context.commit('search_text', search_text);
      let search_text_save = search_text; 
      setTimeout(() => {
        if (search_text_save === context.state.search.text) {
          context.commit('go_page', 1);
          context.dispatch('load_gestures');
        }
      }, 500);
    },
    go_page(context, num) {
      context.commit('go_page', num);
      context.dispatch('load_gestures')
    },
    login(context, credentials) {
      context.commit("start_loading");
      service.login(credentials).then(jwt_payload => {
        context.commit('jwt_payload', jwt_payload);
        context.commit('logging_in', false);
        context.commit("stop_loading");
      }).catch(() => {
        context.commit("stop_loading");
        context.commit('notif', notif_ko_login);
      });
    }
  },
});

