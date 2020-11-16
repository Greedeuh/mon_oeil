<template>
  <div id="app">
    <loading :active.sync="loading" :is-full-page="true" />
    <notifications group="app" />

    <button id="editor_mode" @click="toggle_editor_mode">EDITOR MODE</button>
    <Viewer id="viewer" :class="{ editor_mode: editor_mode }" />
    <Editor v-if="editor_mode" id="editor" />
  </div>
</template>

<script>
import Loading from 'vue-loading-overlay';
import 'vue-loading-overlay/dist/vue-loading.css';
import "vue-select/dist/vue-select.css";

import { mapGetters } from 'vuex'

import Viewer from "./components/Viewer.vue";
import Editor from "./components/Editor.vue";

export default {
  name: "App",
  components: {
    Viewer,
    Editor,
    Loading
  },
  methods: {
    toggle_editor_mode() {
      this.$store.commit("toggle_editor_mode");
    },
  },
  created() {
    this.$store.dispatch('load_all_gestures')
    this.$store.subscribe((mutation, state) => {
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
    })

  },
  computed: {
    ...mapGetters(['editor_mode', 'loading'])
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
