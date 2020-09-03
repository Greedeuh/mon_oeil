<template>
  <div id="picture">
    <div id="img">
      <slot name="img"></slot>
    </div>
    <div id="langs">
      <slot name="langs"></slot>
    </div>
  </div>
</template>

<script>
import Lang from "./Lang.vue";
import { delete_picture } from "../service";

export default {
  name: "Picture",
};
</script>

<style scoped>
#picture {
  position: relative;
}

#langs {
  z-index: 10;
  position: sticky;
}

#img {
  position: absolute;
  left: 0;
  z-index: 0;
}
</style>
