<template>
  <div>
    <div id="picture">
      <Lang
        class="langs"
        v-for="lang in picture.langs"
        :key="lang"
        :lang="lang"
      />
      <img id="picture-img" src="https://picsum.photos/300" />
    </div>
  </div>
</template>

<script>
import Lang from "./Lang.vue";

export default {
  name: "Picture",
  components: {
    Lang,
  },
  props: {
    picture: Object,
  },
  methods: {
    delete_picture() {},
  },
};
</script>

<style scoped>
#picture {
  position: relative;
}

.langs {
  z-index: 10;
  position: sticky;
}

#picture-img {
  position: absolute;
  left: 0;
  z-index: 0;
}

.delete {
  position: relative;
  width: 100%;
}

.delete button {
  position: absolute;
  z-index: 10;
  right: 20px;
}
</style>
