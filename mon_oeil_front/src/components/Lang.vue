<template>
  <img :src="src" />
</template>

<script>
export default {
  name: "Lang",
  props: {
    lang: String,
  },
  computed: {
    src() {
      return "https://www.countryflags.io/" + this.lang + "/flat/32.png";
    },
  },
};
</script>

<style scoped>
img {
  margin: -6px -1px;
  margin-right: 2px;
}
</style>
