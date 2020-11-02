<template>
  <div>
    <div id="gesture">
      <div id="left">
        <div id="imgs">
          <Picture
            v-for="picture in gesture.pictures"
            :key="picture.id"
            :picture="picture"
          />
        </div>
      </div>
      <div id="right">
        <div id="descriptions">
          <Description
            v-for="d in gesture.descriptions"
            :key="d.value"
            :description="d"
          />
        </div>
        <div id="meaning">
          <Meaning v-for="m in gesture.meanings" :key="m.value" :meaning="m" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import Description from "./Description.vue";
import Meaning from "./Meaning.vue";
import Picture from "./Picture.vue";

export default {
  name: "Gesture",
  components: {
    Description,
    Meaning,
    Picture,
  },
  props: {
    gesture: Object,
    admin: Boolean,
  },
  data: function () {
    return { meaning_index: 0 };
  },
  computed: {
    meaning() {
      return this.data.meanings[this.meaning_index];
    },
  },
};
</script>

<style scoped>
#gesture {
  display: grid;
  grid-template-columns: 340px 1fr;

  padding: 20px;
  min-height: 300px;
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
