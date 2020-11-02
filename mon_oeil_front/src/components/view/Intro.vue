<template>
  <div class="intro">
    <h1>Mon œil !</h1>
    <img src="logo.jpg" />
    <p>Les gestes des étudiants français et colombiens</p>
  </div>
</template>

<script>
export default {
  name: "Intro",
  components: {},
};
</script>

<style scoped>
.intro {
  text-align: center;
  height: 100vh;
}

h1 {
  font-family: BrushScript;
  font-weight: lighter;
  color: #009999;
  font-size: 20vh;
  padding: 9vh 0 0 0;
  margin: 0;
}

img {
  height: 45vh;
}

p {
  font-size: 3vh;
  margin: 6vh 0 0 0;
}
</style>
