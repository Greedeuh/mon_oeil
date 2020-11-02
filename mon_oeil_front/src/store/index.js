import Vue from "vue";
import Vuex from "vuex";

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

export default new Vuex.Store({
  state: {
    gestures: mock_gestures,
    editor_mode: true,
    selected_gesture_id: null,
    loading: false,
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
  },
  actions: {
    del_gesture(context, id) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");

        id;
      }, 1000);
    },
    del_description(context, id) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");

        id;
      }, 1000);
    },
    update_description(context, { id, content }) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");

        id;
        content;
      }, 1000);
    },
    add_description_meaning(context, id) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");

        id;
      }, 1000);
    },
    del_meaning(context, id) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");

        id;
      }, 1000);
    },
    update_meaning(context, { id, content }) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");
        id;
        content;
      }, 1000);
    },
    del_picture(context, id) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");

        id;
      }, 1000);
    },
    update_picture(context, { id, content }) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");

        id;
        content;
      }, 1000);
    },
    upload_picture(context, id) {
      context.commit("start_loading");
      setTimeout(() => {
        context.commit("stop_loading");

        id;
      }, 1000);
    },
  },
});
