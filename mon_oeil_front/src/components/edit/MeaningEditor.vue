<template>
  <div class="meaning">
    <button @click="del" class="red-button">Supprimer</button>
    <LangsSelector v-model="langs" />
    <textarea v-model="value"></textarea><br/>
    <button v-if="!same_inner" @click="update" class="classic-button">Enregistrer</button>
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
        new_meaning: { value: this.value, langs: this.langs },
      });
    },
  },
};
</script>

<style scoped>
.meaning {
  margin: 5px;
  padding: 5px;
  border: 1px solid rgba(0, 0, 0, 0.15);
  border-radius: 4px;
}
.meaning:hover {
  border: 1px solid rgba(0, 0, 0, 0.25);
}
</style>
