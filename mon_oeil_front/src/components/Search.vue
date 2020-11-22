<template>
  <div class="search">
      <input class="bar" type="search" :value="search.text" @keyup="text_change" @search="text_change">
  </div>
</template>

<script>
import { mapGetters } from "vuex";

export default {
  name: "Search",
  components: {  },
  methods: {
    text_change(event) {
        this.$store.dispatch("search_text", event.target.value);
    },
  },
  computed: {
    ...mapGetters(["search"]),
  },
};
</script>

<style scoped>
.search {
  text-align: center;
  margin-bottom: 50px;;
}

.bar {
  width: 50%;
  margin: 0 auto;
}
</style>
