<template>
  <div>
    <input type="text" v-model="val" />
    <input type="text" v-model="langs" />
    <button @click="save()">Save description</button>
  </div>
</template>

<script>
import { add_description } from "../service";

export default {
  name: "DescriptionEditor",
  props: {
    id_gesture: String,
    description: Object,
  },
  datas() {
    if (this.description) {
      // on update take existings values
      return {
        val: this.description.val,
        langs: this.description.langs,
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
      let description = { val: this.val, langs: this.langs.split(",") };

      if (this.description) {
        // on update
        description.id = this.description.id;
        // TODO
      } else {
        add_description(this.id_gesture, description).then(() =>
          this.$emit("add")
        );
      }
    },
  },
};
</script>

<style scoped>
</style>
