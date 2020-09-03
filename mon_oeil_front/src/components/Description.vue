<template>
  <div id="description">
    <div :v-if="admin" class="delete">
      <button @click="delete_description()">Delete</button>
    </div>
    <div id="langs">
      <Lang v-for="lang in description.langs" :key="lang" :lang="lang" />
    </div>
    <p>{{ description.value }}</p>
    <div id="meanings">
      <Meaning v-for="m in description.meanings" :key="m.value" :meaning="m" />
      <NewMeaning :id_description=" description.id" />
    </div>
  </div>
</template>

<script>
import Lang from "./Lang.vue";
import Meaning from "./Meaning.vue";
import { delete_description } from "../service";
import NewMeaning from "./NewMeaning";

export default {
  name: "Description",
  components: {
    Lang,
    Meaning,
    NewMeaning,
  },
  props: {
    description: Object,
    admin: Boolean,
  },
  methods: {
    delete_description() {
      delete_description(this.description.id).then(() =>
        this.$emit("delete", this.description.id)
      );
    },
  },
};
</script>

<style scoped>
p {
  font-style: italic;
  color: grey;
  margin: 0;
  margin-bottom: 10px;
}

#description {
  margin-bottom: 20px;
}

.delete {
  position: relative;
  width: 100%;
}

.delete button {
  position: absolute;
  right: -40px;
}
</style>
