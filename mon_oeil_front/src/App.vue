<template>
  <div id="app">
    <Menu />
    <div class="gestures">
      <div v-for="(gesture, index) in gestures" :key="gesture.id">
        <NewGesture />
        <Gesture class="gesture" :gesture="gesture" :admin="user" />
        <hr v-if="index != gestures.length -1" />
      </div>
    </div>
  </div>
</template>

<script>
import Gesture from "./components/Gesture.vue";
import Menu from "./components/Menu.vue";
import NewGesture from "./components/NewGesture.vue";

export default {
  name: "App",
  components: {
    Gesture,
    Menu,
    NewGesture,
  },
  data: function () {
    return {
      user: "",
      gestures: [
        {
          id: "Bien!1",
          descriptions: [
            {
              id: "Bien!1",
              value: "Le pouce est vers le haut",
              langs: ["fr", "us"],
              meanings: [
                {
                  value: "Super!, Bien joué!, Nickel ou simplement Bien.",
                  langs: ["fr", "us"],
                },
                {
                  value: "Bien joué!, Nickel ou simplement Bien.",
                },
              ],
            },
            {
              value: "Le pouce est vers le bas",
              langs: ["fr", "us"],
              meanings: [
                {
                  value: "C'est perdu!, Pas cool... c'est mort.",
                },
              ],
            },
          ],

          meanings: [
            {
              value: " Super!, Bien joué!, Nickel ou simplement Bien.",
              langs: ["us"],
            },
          ],
          pictures: [{ id: "xqxq", langs: ["fr", "us"] }],
        },
        {
          id: "Bien!",
          descriptions: [
            {
              value: "Le pouce est vers le haut",
              langs: ["fr", "us"],
              meanings: [
                {
                  value: "Super!, Bien joué!, Nickel ou simplement Bien.",
                  langs: ["fr", "us"],
                },
                {
                  value: "Bien joué!, Nickel ou simplement Bien.",
                },
              ],
            },
            {
              value: "Le pouce est vers le bas",
              langs: ["fr", "us"],
              meanings: [
                {
                  value: "C'est perdu!, Pas cool... c'est mort.",
                },
              ],
            },
          ],

          meanings: [
            {
              value: " Super!, Bien joué!, Nickel ou simplement Bien.",
              langs: ["us"],
            },
          ],
          pictures: [{ id: "xqxq", langs: ["fr", "us"] }],
        },
      ],
    };
  },
};
</script>

<style>
body {
  margin: 0;
  background-color: #f3f2e2;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  font-size: 19px;
}

.gestures {
  margin: 0 auto;
  border-radius: 3px;
  -moz-border-radius: 3px;
  -webkit-border-radius: 3px;
  /* border: 3px solid #ebdccb; */
  background-color: white;
  -webkit-box-shadow: 0px 0px 6px 0px rgba(0, 0, 0, 0.33);
  -moz-box-shadow: 0px 0px 6px 0px rgba(0, 0, 0, 0.33);
  box-shadow: 0px 0px 6px 0px rgba(0, 0, 0, 0.33);
  width: 900px;
}

hr {
  width: 50%;
  border-top: none;
  border-color: rgba(0, 0, 0, 0.2);
  margin-bottom: 20px;
}
</style>
