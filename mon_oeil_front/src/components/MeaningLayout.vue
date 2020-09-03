<template>
  <div>
    <div id="langs">
      <slot name="langs"></slot>
    </div>
    <div id="value">
      <slot name="value"></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: "MeaningLayout",
};
</script>

<style scoped>
p {
  margin: 0;
  margin-bottom: 10px;
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
