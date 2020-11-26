<template>
  <div class="picture">
    <LangsSelector v-model="langs" />
    <img id="picture-img" v-if="file" :src="preview_file" />
    <input type="file" ref="input_file" v-on:change="select_file" accept="image/png, image/jpeg"/>
    <span :class="{ bad_file }">File must be a .png or .jpeg and 300x300px</span>
    <button v-if="file && !bad_file" @click="add_file">Ajouter</button>
  </div>
</template>

<script>
import LangsSelector from "./LangsSelector";



export default {
  name: "PictureCreator",
  components: { LangsSelector },
  props: {
    id_gesture: String,
  },
  data: function () {
    return {
      file: undefined,
      preview_file: undefined,
      bad_file: false,
      langs: [],
    }
  } ,
  methods: {
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
    add_file() {
      this.$store.dispatch("add_picture", { id_gesture:this.id_gesture, langs: this.langs, file: this.file });
      this.clear();
    },
    clear(){
      this.$refs.input_file.value = '';
      this.file = undefined;
      this.preview_file = undefined;
      this.bad_file = false;
      this.langs = [];
    }
  },
};
</script>

<style scoped>
.bad_file {
  border: 1px solid red; 
}

.picture {
  margin: 5px;
  padding: 5px;
  border: 1px solid rgba(0, 0, 0, 0.15);
  border-radius: 4px;
}

.picture:hover {
  border: 1px solid rgba(0, 0, 0, 0.25);
}
</style>
