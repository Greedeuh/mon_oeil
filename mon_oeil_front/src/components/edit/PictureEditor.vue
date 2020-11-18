<template>
  <div>
    <button @click="del">Supprimer</button>
    <LangsSelector v-model="langs" />
    <button v-if="!same_inner" @click="update_meta">Enregitrer</button>
    <img id="picture-img" :src="file ? preview_file : picture.url" />
    <input type="file" ref="input_file" v-on:change="select_file" accept="image/png, image/jpeg"/>
    <span :class="{ bad_file }">File must be a .png or .jpeg and 300x300px</span>
    <button v-if="file && !bad_file" @click="update_file">Remplacer</button>
  </div>
</template>

<script>
import LangsSelector from "./LangsSelector";

import _ from "lodash";

export default {
  name: "PictureEditor",
  components: { LangsSelector },
  props: {
    picture: Object,
  },
  computed: {
    same_inner() {
      let props_langs = this.picture.langs
        ? [...this.picture.langs]
        : [];

      return _.isEqual(this.langs, props_langs);
    },
  },
  data: function () {
    return {
      file: undefined,
      preview_file: undefined,
      bad_file: false,
      langs: this.picture.langs ? [...this.picture.langs] : [],
    };
  },
  methods: {
    del() {
      this.$store.dispatch("del_picture", this.picture.id);
    },
    update_meta() {
      this.$store.dispatch("update_picture_meta", { id:this.picture.id, new_picture_meta: { langs: this.langs } });
    },
    select_file(){
      this.file = this.$refs.input_file.files[0];
      this.preview_file = URL.createObjectURL(this.file);
      let img = new Image();
      img.onload = () => {
        if (img.width != 300 && img.height != 300) {
          this.bad_file = true;
        } else {
          this.bad_file = false;
        }
      }
      img.src = this.preview_file;

    },
    update_file() {
      this.$store.dispatch("update_picture_file", {id:this.picture.id, new_picture_file: this.file});
    },
  },
};
</script>

<style scoped>
.bad_file {
  border: 1px solid red; 
}</style>
