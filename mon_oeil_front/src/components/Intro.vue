<template>
  <div class="intro">
    <div class="pres">
      <h1>Mon œil !</h1>
      <img src="logo.png" />
      <p>Les gestes des étudiants français et colombiens</p>
    </div>
    <div class="links">
      <router-link to="/about" tag="button" class="classic-button">A propos</router-link>
      <router-link to="/contribute" tag="button" class="classic-button">Contribuer</router-link>
    </div>
  </div>
</template>

<script>
export default {
  name: "Intro",
  components: {},
};
</script>

<style scoped>
.pres {
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

.links {
  text-align: center;
}

.links button {
  font-size:20px;
  margin: 40px 20px;
  margin-top: 0px;
}
</style>
