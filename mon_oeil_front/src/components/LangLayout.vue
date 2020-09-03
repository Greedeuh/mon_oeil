<template>
  <div id="img">
    <slot></slot>
  </div>
</template>

<script>
export default {
  name: "LangLayout",
};
</script>

<style scoped>
#img {
  margin: -6px -1px;
  margin-right: 2px;
}
</style>
