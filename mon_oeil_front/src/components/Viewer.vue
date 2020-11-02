<template>
  <div class="viewer">
    <div class="content">
      <Intro class="intro" />
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
    </div>
  </div>
</template>

<script>
import Intro from "./view/Intro.vue";
import Gesture from "./view/Gesture.vue";

import { mapGetters } from "vuex";

export default {
  name: "Viewer",
  components: {
    Intro,
    Gesture,
  },
  methods: {
    select(gesture) {
      this.$store.commit("select_gesture", gesture.id);
    },
  },
  computed: {
    ...mapGetters(["gestures"]),
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
}

.content {
  margin: 0 auto;
  max-width: 900px;
  padding: 0 70px;
  background-image: url(../assets/dot_background.png);
  background-repeat: repeat-y;
  background-position: right;
}

hr.dot_hr {
  height: 32px;
  width: 50%;
  border-top: none;
  border: none;
  background-image: url(../assets/dot_hr.png);
}
</style>
