<template>
  <div id="app">
    <loading :active.sync="loading" :is-full-page="true" />
    <notifications group="app" />
    <Login v-if="user.logging_in"/>
    <router-view/>
  </div>
</template>

<script>
import Loading from 'vue-loading-overlay';
import 'vue-loading-overlay/dist/vue-loading.css';
import "vue-select/dist/vue-select.css";

import { mapGetters } from 'vuex'

import Login from "./components/Login.vue";

const key_pressed = [];

export default {
  name: "App",
  components: {
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
}

a {
  text-decoration: none;
}

a:visited {
  color: #2c3e50;
}

a:hover {
  text-decoration: underline;  
}

.classic-button {
	box-shadow:inset 0px 39px 0px -24px #07adad;
	background:linear-gradient(to bottom, #009999 5%, #03a3a3 100%);
	background-color:#009999;
	border-radius:4px;
	border:1px solid #ffffff;
	display:inline-block;
	cursor:pointer;
	color:#ffffff;
	font-family:Arial;
	padding:6px 15px;
	text-decoration:none;
	text-shadow:0px 1px 0px #006161;
}

.classic-button:hover {
	background:linear-gradient(to bottom, #03a3a3 5%, #009999 100%);
	background-color:#03a3a3;
}

.classic-button:active {
	position:relative;
	top:1px;
}

.red-button {
	box-shadow:inset 0px 39px 0px -24px #e74949;
	background:linear-gradient(to bottom, #dd4141 5%, #ca3939 100%);
	background-color:#dd4141;
	border-radius:4px;
	border:1px solid #ffffff;
	display:inline-block;
	cursor:pointer;
	color:#ffffff;
	font-family:Arial;
	padding:6px 15px;
	text-decoration:none;
	text-shadow:0px 1px 0px #8a2a2a;
}

.red-button:hover {
	background:linear-gradient(to bottom, #ca3939 5%, #dd4141 100%);
	background-color:#ca3939;
}

.red-button:active {
	position:relative;
	top:1px;
}

.vue-select {
  max-width: 500px;
  width: 100%;
  margin: 0 10px;
}

textarea {
  max-width: 500px;
  width: 100%;
  margin: 0 10px;
}
</style>
