<template>
  <div class="pagination" v-if="search.total_pages !== 1">
    <span v-if="1 !== search.page" @click="previous">&lt;</span>
    <span v-for="p in search.total_pages" :key="p" @click="select(p)" :class="{selected: p === search.page }">&nbsp;{{ p }}&nbsp;</span>
    <span v-if="search.total_pages !== search.page" @click="next">&gt;</span>
  </div>
</template>

<script>
import { mapGetters } from "vuex";

export default {
  name: "Pagination",
  components: {  },
  methods: {
    previous() {
      this.$store.dispatch("go_page", this.search.page - 1);
    },
    next() {
      this.$store.dispatch("go_page", this.search.page + 1);
    },
    select(num) {
      if (num === this.search.page) return;
      this.$store.dispatch("go_page", num);
    }
  },
  computed: {
    ...mapGetters(["search"]),
  },
};
</script>

<style scoped>
.pagination {
  text-align: center;
}

.selected {
  text-decoration: underline;
}
</style>
