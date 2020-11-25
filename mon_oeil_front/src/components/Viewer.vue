<template>
  <div class="viewer" ref="el">
    <div class="content">
      <Intro class="intro" />
      <Search class="search"/>
      <div class="gestures">
        <div v-for="(gesture, index) in gestures" :key="gesture.id">
          <Gesture
            @click.native="select(gesture)"
            class="gesture"
            :gesture="gesture"
          />
          <hr v-if="index != gestures.length - 1" class="dot_hr" />
        </div> 
      </div>
      <Pagination />
    </div>
    <span v-if="gestures.length == 0">Aucun résultat :(</span>
  </div>
</template>

<script>
import Intro from "./Intro.vue";
import Gesture from "./gestures/Gesture.vue";
import Search from "./Search.vue"
import Pagination from "./Pagination.vue"

import { mapGetters } from "vuex";

export default {
  name: "Viewer",
  components: {
    Intro,
    Gesture,
    Search,
    Pagination
  },
  methods: {
    select(gesture) {
      this.$store.commit("select_gesture", gesture.id);
    },
  },
  computed: {
    ...mapGetters(["gestures"]),
  },
  created() {
    if (this.gestures.length <= 0) {
      this.$store.dispatch('load_gestures')
    }

    this.$store.subscribe((mutation, state) => {
      if(mutation.type === 'go_page' && state.search_count > 1) {
        this.$refs.el.scroll({
          top: window.innerHeight,
          left: 0,
          behavior: 'smooth'
        }); 
      }
    })
  },
};
</script>

<style scoped>
.viewer {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  font-size: 19px;

  width: 100%;
  height: 100vh;
  overflow: auto;
  text-align: center;
}

.content {
  margin: 0 auto;
  max-width: 900px;
  padding: 0 70px;
  background-image: url(../assets/dot_background.png);
  background-repeat: repeat-y;
  background-position: right;
  text-align: initial;
}

hr.dot_hr {
  height: 32px;
  width: 50%;
  border-top: none;
  border: none;
  background-image: url(../assets/dot_hr.png);
}
</style>
