<template>
  <div>
    <div id="menu">
      <button @click="login()">Login</button>
      <button @click="add_gesture()">Add</button>
    </div>
  </div>
</template>

<script>
import { login, add_gesture } from "../service";

export default {
  name: "Menu",
  components: {},
  props: {},
  methods: {
    login() {
      login("user_test", "password_test");
    },
    add_gesture() {
      add_gesture({
        tags: ["heho"],
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
                langs: []
              },
            ],
          },
          {
            value: "Le pouce est vers le bas",
            langs: ["fr", "us"],
            meanings: [
              {
                value: "C'est perdu!, Pas cool... c'est mort.",
                langs:[]
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
        pictures: [{  langs: ["fr", "us"] }],
      });
    },
  },
};
</script>

<style scoped>
#menu {
  width: 100%;
  background-color: #32b4a9;
  height: 70px;
  margin-bottom: 30px;
  -webkit-box-shadow: 0px 0px 6px 0px rgba(0, 0, 0, 0.33);
  -moz-box-shadow: 0px 0px 6px 0px rgba(0, 0, 0, 0.33);
  box-shadow: 0px 0px 6px 0px rgba(0, 0, 0, 0.33);
}
</style>
