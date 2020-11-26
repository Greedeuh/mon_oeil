<template>
  <div id="editor">
    <h1>Editeur <button class="add-gesture classic-button" @click="add_gesture"> + Nouveau geste</button></h1> 
    <GestureEditor v-if="selected_gesture" :gesture="selected_gesture"/>
  </div>
</template>

<script>
import GestureEditor from "./edit/GestureEditor.vue";

import { mapGetters } from "vuex";

export default {
  name: "Editor",
  components: { GestureEditor },
  methods: {
    add_gesture(){
      this.$store.dispatch("add_gesture", {
        tags: [],
      });
    }
  },
  computed: {
    ...mapGetters(["selected_gesture"]),
  },
};
</script>

<style scoped>
#editor {
  height: 100vh;
  overflow: auto;
}

h1 {
  display: flex;
}

.add-gesture {
  margin: 7px 25px;
}
</style>
