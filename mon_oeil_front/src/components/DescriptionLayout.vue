<template>
  <div id="description">
    <div id="langs">
      <slot name="langs"></slot>
    </div>
    <div>
      <slot name="value"></slot>
    </div>
    <div id="meanings">
      <slot name="meanings"></slot>
    </div>
  </div>
</template>

<script>
export default {
  name: "DescriptionLayout",
};
</script>

<style scoped>
p {
  font-style: italic;
  color: grey;
  margin: 0;
  margin-bottom: 10px;
}

#description {
  margin-bottom: 20px;
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
