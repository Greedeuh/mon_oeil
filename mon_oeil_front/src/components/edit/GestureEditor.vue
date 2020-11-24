<template>
  <div class="gesture">
    <button @click="del">Supprimer</button>
      <v-select multiple push-tags taggable v-model="inner_tags"/>
    <button v-if="!same_inner" @click="update_gesture">Enregistrer</button>
    <DescriptionEditor
      v-for="description in gesture.descriptions"
      :key="description.id"
      :description="{ ...description }"
    />
    <button @click="add_description">Ajouter une Description</button>
    <MeaningEditor
      v-for="meaning in gesture.meanings"
      :key="meaning.id"
      :meaning="meaning"
      class="meaning"
    />
    <button @click="add_meaning">Ajouter un Meaning</button>

    <PictureEditor v-for="picture in gesture.pictures" :key="picture.id" :picture="picture"/>
    <PictureCreator :id_gesture="gesture.id" />
  </div>
</template>

<script>
import DescriptionEditor from "./DescriptionEditor.vue";
import MeaningEditor from "./MeaningEditor";
import PictureEditor from "./PictureEditor";
import PictureCreator from "./PictureCreator";

import vSelect from "vue-select";
import _ from "lodash";

export default {
  name: "Editor",
  components: { DescriptionEditor, MeaningEditor, PictureEditor,PictureCreator, vSelect },
  props: {
    gesture: Object
  },
  data: function () {
    return {
      inner_tags: this.gesture.tags ? [...this.gesture.tags] : [],
    };
  },
  watch: {
    gesture: function(gesture) {
      this.inner_tags=gesture.tags ? [...gesture.tags] : [];
    }
  },
  methods: {
    del(){
      this.$store.dispatch("del_gesture", this.gesture.id);
    },
    update_gesture(){
      this.$store.dispatch("update_gesture", { id: this.gesture.id, new_gesture: {
        tags: this.inner_tags,
      } });
    },
    add_meaning() {
      this.$store.dispatch("add_gesture_meaning", {
        id_gesture: this.gesture.id,
        new_meaning: { value: "", langs: ["fr"] },
      });
    },
    add_description() {
      this.$store.dispatch("add_description", {
        id_gesture: this.gesture.id,
        new_description: { value: "", langs: ["fr"] },
      });
    }
  },
  computed: {
    same_inner() {
      let props_tags = this.gesture.tags
        ? [...this.gesture.tags]
        : [];

      return _.isEqual(this.inner_tags, props_tags);
    },
  },
};
</script>

<style scoped>
.gesture {
  padding: 0 20px;
}
</style>
