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

export default new Vuex.Store({
  state: {
    gestures: mock_gestures,
    editor_mode: true,
    selected_gesture_id: null,
    loading: false,
    notif: null
  },
  getters: {
    gestures(state) {
      return state.gestures;
    },
    selected_gesture(state) {
      return state.gestures.find(
        (gesture) => gesture.id === state.selected_gesture_id
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
    }
  },
  mutations: {
    toggle_editor_mode(state) {
      state.editor_mode = !state.editor_mode;
    },
    select_gesture(state, selected_gesture_id) {
      state.selected_gesture_id = selected_gesture_id;
    },
    start_loading(state) {
      state.loading = true;
    },
    stop_loading(state) {
      state.loading = false;
    },
    load_gestures(state, gestures) {
      state.gestures = gestures;
    },
    notif(state, notif) {
      state.notif = notif;
    }
  },
  actions: {
    load_all_gestures(context) {
      service.get_all_gestures().then(gestures => context.commit("load_gestures",gestures)).catch(() => context.commit('notif', notif_ko))
      context.commit("stop_loading");
    },
    del_gesture(context, id) {
      context.commit("start_loading");
      service.delete_gesture(id).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    del_description(context, id) {
      context.commit("start_loading");
      service.delete_description(id).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    del_meaning(context, id) {
      context.commit("start_loading");
      service.delete_meaning(id).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    del_picture(context, id) {
      context.commit("start_loading");
      service.delete_picture(id).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    update_description(context, { id, new_description }) {
      context.commit("start_loading"); 
      service.put_description(id, new_description).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    update_meaning(context, { id, new_meaning }) {
      service.put_meaning(id, new_meaning).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    update_picture_meta(context, { id, new_picture_meta }) {
      service.put_picture_meta(id, new_picture_meta).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    update_picture_file(context, { id, new_picture_file }) {
      service.put_picture_file(id, new_picture_file).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    add_description_meaning(context, {id_description, new_meaning}) {
      context.commit("start_loading"); 
      service.post_description_meaning(id_description, new_meaning).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    add_gesture_meaning(context, {id_gesture, new_meaning}) {
      context.commit("start_loading"); 
      service.post_gesture_meaning(id_gesture, new_meaning).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
    add_description(context, {id_gesture, new_description}) {
      context.commit("start_loading"); 
      service.post_description(id_gesture, new_description).then(() => {
        context.dispatch('load_all_gestures');
        context.commit('notif', notif_ok);
      }).catch(() => {
        context.commit('notif', notif_ko);
        context.commit("stop_loading");
      });
    },
  },
});

