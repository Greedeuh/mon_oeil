<template>
  <div>
    <button @click="del">Supprimer</button>
    <LangsSelector v-model="langs" />
    <input type="text" v-model="value" />
    <button v-if="!same_inner" @click="update">Enregistrer</button>
  </div>
</template>

<script>
import LangsSelector from "./LangsSelector";

import _ from "lodash";

export default {
  name: "MeaningEditor",
  components: { LangsSelector },
  props: {
    meaning: Object,
  },
  data: function () {
    return {
      langs: this.meaning.langs ? [...this.meaning.langs] : [],
      value: this.meaning.value,
    };
  },
  computed: {
    same_inner() {
      let props_langs = this.meaning.langs ? [...this.meaning.langs] : [];
      return (
        _.isEqual(this.langs, props_langs) && this.value === this.meaning.value
      );
    },
  },
  methods: {
    del() {
      this.$store.dispatch("del_meaning", this.meaning.id);
    },
    update() {
      this.$store.dispatch("update_meaning", {
        id: this.meaning.id,
        content: { value: this.value, langs: this.langs },
      });
    },
  },
};
</script>

<style scoped></style>
