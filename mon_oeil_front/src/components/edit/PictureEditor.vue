<template>
  <div>
    <button @click="del">Supprimer</button>
    <LangsSelector v-model="langs" />
    <button @click="update">Enregitrer</button>
    <img id="picture-img" src="https://picsum.photos/300" />
    <input type="file" />
    <button @click="upload">Remplacer</button>
  </div>
</template>

<script>
import LangsSelector from "./LangsSelector";

export default {
  name: "PictureEditor",
  components: { LangsSelector },
  props: {
    picture: Object,
  },
  data: function () {
    return {
      langs: this.picture.langs ? [...this.picture.langs] : [],
    };
  },
  methods: {
    del() {
      this.$store.dispatch("del_picture", this.picture.id);
    },
    update() {
      this.$store.dispatch("update_picture", this.picture.id);
    },
    upload() {
      this.$store.dispatch("upload_picture", this.picture.id);
    },
  },
};
</script>

<style scoped></style>
