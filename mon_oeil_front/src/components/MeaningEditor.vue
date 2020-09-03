<template>
  <div>
    <input type="text" v-model="val" />
    <input type="text" v-model="langs" />
    <button @click="save()">Save meaning</button>
  </div>
</template>

<script>
import { add_gesture_s_meaning, add_description_s_meaning } from "../service";

export default {
  name: "NewMeaning",
  props: {
    id_gesture: String,
    id_description: String,
    meaning: Object,
  },
  datas() {
    if (this.meaning) {
      // on update take existings values
      return {
        val: this.meaning.val,
        langs: this.meaning.langs,
      };
    } else {
      // on add take default values
      return {
        val: "",
        langs: "",
      };
    }
  },
  methods: {
    save() {
      let meaning = { val: this.val, langs: this.langs.split(",") };

      if (this.meaning) {
        // on update
        meaning.id = this.meaning.id;
        // TODO
      } else {
        let res;
        if (this.id_gesture) {
          // on add into gesture
          res = add_gesture_s_meaning(this.id_gesture, meaning);
        } else if (this.id_description) {
          // on add into description
          res = add_description_s_meaning(this.id_description, meaning);
        } else {
          return console.log("Need id to create meaning");
        }
        res.then(() => this.$emit("add"));
      }
    },
  },
};
</script>

<style scoped>
</style>
