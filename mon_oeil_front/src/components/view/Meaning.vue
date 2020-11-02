<template>
  <div id="meaning-langs">
    <Lang v-for="lang in meaning.langs" :key="lang" :lang="lang" />
    <p>{{ meaning.value }}</p>
  </div>
</template>

<script>
import Lang from "./Lang.vue";

export default {
  name: "Meaning",
  components: {
    Lang,
  },
  props: {
    meaning: Object,
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
