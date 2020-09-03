<template>
  <div>
    <input type="text" v-model="gesture.tags" />
    <div v-for="gesture in gestures" :key="gesture.id">
      <NewDescription :description="description" />
    </div>
    <div v-for="gesture in gestures" :key="gesture.id">
      <NewDescription :description="description" />
    </div>
    <button @click="save()">Save gesture</button>
  </div>
</template>

<script>
import { add_gesture } from "../service";
import DescriptionEditor from "./NewDescription";
import MeaningEditor from "./NewMeaning";
import PictureEditor from "./NewPicture";
import LangEditor from "./NewPicture";

export default {
  name: "GestureEditor",
  components: { DescriptionEditor, MeaningEditor, PictureEditor, LangEditor },
  props: {
    gesture: Object,
  },
  methods: {
    save() {
      let gesture = {
        tags: this.tags,
        descriptions: [],
        meanings: [],
        pictures: [],
      };

      if (this.gesture) {
        // on update
        gesture.id = this.gesture.id;

        // TODO
      } else {
        add_gesture(gesture).then(() => this.$emit("add"));
      }
    },
  },
};
</script>

<style scoped>
</style>
