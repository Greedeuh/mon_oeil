<template>
  <div id="app">
    <loading :active.sync="loading" :is-full-page="true" />
    <notifications group="app" />
    <Viewer id="viewer" :class="{ editor_mode: editor_mode }" />
    <Editor v-if="editor_mode" id="editor" />
    <Login v-if="user.logging_in"/>
  </div>
</template>

<script>
import Loading from 'vue-loading-overlay';
import 'vue-loading-overlay/dist/vue-loading.css';
import "vue-select/dist/vue-select.css";

import { mapGetters } from 'vuex'

import Viewer from "./components/Viewer.vue";
import Editor from "./components/Editor.vue";
import Login from "./components/Login.vue";

const key_pressed = [];

export default {
  name: "App",
  components: {
    Viewer,
    Editor,
    Loading,
    Login
  },
  methods: {
    on_notif(mutation, state) {
      if (mutation.type === "notif") {
        let type;
        if (state.notif.success) {
          type = "success";
        } else {
          type = "error";
        }
        this.$notify({
          group: 'app',
          text: state.notif.msg,
          type
        });
      }
    },
    on_key_down(event){
      // LOGIN
      this.on_key_down_login(event)
      this.on_key_down_editor_mode(event)
      
    },
    on_key_down_login(event){
      if (event.key === 'Alt' || event.key === 'l' || event.key === 'L') {
        key_pressed[event.key] = true
        if (key_pressed['Alt'] && (key_pressed['l'] || key_pressed['L']) && !this.user.logging_in) {
          this.$store.commit('logging_in', true);
        }
      }
    },on_key_down_editor_mode(event){
      if (event.key === 'Alt' || event.key === 'm' || event.key === 'M') {
        key_pressed[event.key] = true
        if (key_pressed['Alt'] && (key_pressed['m'] || key_pressed['M'])) {
          this.$store.commit("toggle_editor_mode");
        }
      }
    },
    on_key_up(event){
      if (event.key === 'Alt' || event.key === 'l' || event.key === 'L' || event.key === 'm' || event.key === 'M') {
        key_pressed[event.key] = false
      }
    }
  },
  created() {
    this.$store.dispatch('load_gestures')

    this.$store.subscribe(this.on_notif);

    window.addEventListener('keydown', this.on_key_down);
    window.addEventListener('keyup', this.on_key_up);
  },
  computed: {
    ...mapGetters(['editor_mode', 'loading', 'user'])
  },
};
</script>

<style>
body {
  margin: 0;
  /* background-color: #f3f2e2; */
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  font-size: 19px;
  display: flex;
}

#editor_mode {
  position: fixed;
}

#viewer.editor_mode {
  width: 50vw;
}

#editor {
  width: 50vw;
}
</style>
