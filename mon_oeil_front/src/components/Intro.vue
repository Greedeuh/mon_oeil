<template>
  <div class="intro">
    <div class="pres">
      <h1>Mon œil !</h1>
      <img src="logo.png" />
      <p>Les gestes des étudiants français et colombiens</p>
    </div>
    <div class="links">
      <router-link to="/about" tag="button">A propos</router-link>
      <button>Contribuer</button>
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
  margin: 40px 20px;
  margin-top: 0px;
	box-shadow:inset 0px 39px 0px -24px #07adad;
	background:linear-gradient(to bottom, #009999 5%, #03a3a3 100%);
	background-color:#009999;
	border-radius:4px;
	border:1px solid #ffffff;
	display:inline-block;
	cursor:pointer;
	color:#ffffff;
	font-family:Arial;
	font-size:20px;
	padding:6px 15px;
	text-decoration:none;
	text-shadow:0px 1px 0px #006161;
}
.links button:hover {
	background:linear-gradient(to bottom, #03a3a3 5%, #009999 100%);
	background-color:#03a3a3;
}
.links button:active {
	position:relative;
	top:1px;
}

</style>
