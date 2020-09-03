<template>
  <div>
    <div id="picture">
      <div :v-if="admin" class="delete">
        <button @click="delete_picture()">Delete</button>
      </div>
      <Lang class="langs" v-for="lang in picture.langs" :key="lang" :lang="lang" />
      <img id="picture-img" src="https://picsum.photos/300" />
    </div>
  </div>
</template>

<script>
import Lang from "./Lang.vue";
import { delete_picture } from "../service";

export default {
  name: "Picture",
  components: {
    Lang,
  },
  props: {
    picture: Object,
    admin: Boolean,
  },
  methods: {
    delete_picture() {
      delete_picture(this.picture.id).then(() =>
        this.$emit("delete", this.picture.id)
      );
    },
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
