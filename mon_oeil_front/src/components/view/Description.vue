<template>
  <div id="description">
    <div id="langs">
      <Lang v-for="lang in description.langs" :key="lang" :lang="lang" />
    </div>
    <p>{{ description.value }}</p>
    <div id="meanings">
      <Meaning v-for="m in description.meanings" :key="m.value" :meaning="m" />
    </div>
  </div>
</template>

<script>
import Lang from "./Lang.vue";
import Meaning from "./Meaning.vue";

export default {
  name: "Description",
  components: {
    Lang,
    Meaning,
  },
  props: {
    description: Object,
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
