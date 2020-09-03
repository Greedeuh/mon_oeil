<template>
  <div>
    <div id="gesture">
      <div id="left">
        <div id="imgs">
          <slot name="pictures"></slot>
        </div>
      </div>
      <div id="right">
        <div id="descriptions">
          <slot name="descriptions"></slot>
        </div>
        <div id="meanings">
          <slot name="meanings"></slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "GestureLayout",
  props: {
    gesture: Object,
    admin: Boolean,
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
</style>
