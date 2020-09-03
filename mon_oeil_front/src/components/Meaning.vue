<template>
  <div id="meaning-langs">
    <div :v-if="admin" class="delete">
      <button @click="delete_meaning()">Delete</button>
    </div>
    <Lang v-for="lang in meaning.langs" :key="lang" :lang="lang" />
    <p>{{ meaning.value }}</p>
  </div>
</template>

<script>
import Lang from "./Lang.vue";
import { delete_meaning } from "../service";

export default {
  name: "Meaning",
  components: {
    Lang,
  },
  props: {
    meaning: Object,
    admin: Boolean,
  },
  methods: {
    delete_meaning() {
      delete_meaning(this.meaning.id).then(() =>
        this.$emit("delete", this.meaning.id)
      );
    },
  },
};
</script>

<style scoped>
p {
  margin: 0;
  margin-bottom: 10px;
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
