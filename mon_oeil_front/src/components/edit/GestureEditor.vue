<template>
  <div class="gesture">
    <button @click="del" class="red-button del">x Supprimer ce geste</button>
    <br/>
      <div class="tags">
        <label>Tags:</label><v-select multiple push-tags taggable v-model="inner_tags" class="vue-select"/>
      <button v-if="!same_inner" @click="update_gesture" class="classic-button">Enregistrer</button>
    </div>
    <div class="descriptions">
      <h3>Descriptions</h3>
      <DescriptionEditor
        v-for="description in gesture.descriptions"
        :key="description.id"
        :description="{ ...description }"
      />
    </div>
    <button @click="add_description" class="classic-button">+ Ajouter une description</button>
    <div class="meanings">
      <h3>Sens global</h3>
      <MeaningEditor
        v-for="meaning in gesture.meanings"
        :key="meaning.id"
        :meaning="meaning"
        class="meaning"
      />
      <button @click="add_meaning" class="classic-button">+ Ajouter un Meaning</button>
    </div>
    <div class="pictures">
      <h3>Images</h3>
      <PictureEditor v-for="picture in gesture.pictures" :key="picture.id" :picture="picture"/>
      <h4>Nouvelle image</h4>
      <PictureCreator :id_gesture="gesture.id" />
    </div>
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

.del {
  margin-bottom: 20px;
}

.tags {
  display: flex;
  align-items: center;
}
</style>
