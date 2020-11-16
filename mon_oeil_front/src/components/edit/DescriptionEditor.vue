<template>
  <div>
    <button @click="del">Supprimer</button>
    <div class="inner">
      <LangsSelector v-model="langs" />
      <input type="text" v-model="value" />
      <button v-if="!same_inner" @click="update">Enregistrer</button>
    </div>
    <MeaningEditor
      v-for="meaning in description.meanings"
      :key="meaning.id"
      :meaning="meaning"
      class="meaning"
    />
    <button @click="add_meaning" class="meaning">Ajouter un Meaning</button>
  </div>
</template>

<script>
import MeaningEditor from "./MeaningEditor";
import LangsSelector from "./LangsSelector";

import _ from "lodash";

export default {
  name: "DescriptionEditor",
  components: { MeaningEditor, LangsSelector },
  props: {
    description: Object,
  },
  data: function () {
    return {
      langs: this.description.langs ? [...this.description.langs] : [],
      value: this.description.value,
    };
  },
  computed: {
    same_inner() {
      let props_langs = this.description.langs
        ? [...this.description.langs]
        : [];

      return (
        _.isEqual(this.langs, props_langs) &&
        this.value === this.description.value
      );
    },
  },
  methods: {
    del() {
      this.$store.dispatch("del_description", this.description.id);
    },
    update() {
      this.$store.dispatch("update_description", {
        id: this.description.id,
        new_description: { value: this.value, langs: this.langs },
      });
    },
    add_meaning() {
      this.$store.dispatch("add_description_meaning", {
        id_description: this.description.id,
        new_meaning: { value: "", langs: ["fr"] },
      });
    },
  },
};
</script>

<style scoped>
.meaning {
  margin-left: 30px;
}
</style>
