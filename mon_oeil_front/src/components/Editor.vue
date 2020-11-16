<template>
  <div id="editor">
    <div id="content" v-if="selected_gesture">
      <h1>Editeur</h1>
      <button @click="del">Supprimer</button>
      <DescriptionEditor
        v-for="description in selected_gesture.descriptions"
        :key="description.id"
        :description="{ ...description }"
      />
      <button @click="add_description">Ajouter une Description</button>
      <MeaningEditor
        v-for="meaning in selected_gesture.meanings"
        :key="meaning.id"
        :meaning="meaning"
        class="meaning"
      />
      <button @click="add_meaning">Ajouter un Meaning</button>

      <PictureEditor v-for="picture in selected_gesture.pictures" :key="picture.id" :picture="picture"/>
    </div>
  </div>
</template>

<script>
import DescriptionEditor from "./edit/DescriptionEditor.vue";
import MeaningEditor from "./edit/MeaningEditor";
import PictureEditor from "./edit/PictureEditor";

import { mapGetters } from "vuex";

export default {
  name: "Editor",
  components: { DescriptionEditor, MeaningEditor, PictureEditor },
  methods: {
    del(){
      this.$store.dispatch("del_gesture", this.selected_gesture.id);
    },add_meaning() {
      this.$store.dispatch("add_gesture_meaning", {
        id_gesture: this.selected_gesture.id,
        new_meaning: { value: "", langs: ["fr"] },
      });
    },
    add_description() {
      this.$store.dispatch("add_description", {
        id_gesture: this.selected_gesture.id,
        new_description: { value: "", langs: ["fr"] },
      });
    },
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

#content {
  padding: 0 20px;
}
</style>
