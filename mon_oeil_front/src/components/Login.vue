<template>
  <div class="login_background" @click="close">
    <div class="login">
      <div v-if="!user_logged" class="connect">
        <form @submit.prevent="login" @click.stop="stop">
          <input v-model="username" class="inputs" type="text" placeholder="username"/><br>
          <input v-model="password" class="inputs" type="passsword" placeholder="password"/><br>
          <input class="inputs submit" type="submit" value="Se connecter" ><br>
        </form>
      </div>
      <div v-else class="connected">
        {{ user.jwt_payload.username }} vous êtes bien connecté :).
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";

export default {
  name: "Login",
  components: {  },
  data() {
    return {
      username: '',
      password: ''
    }
  },
  methods: {
   login(){
     this.$store.dispatch('login', { username: this.username, password: this.password })
   },
   on_close(e){
      if (e.key === 'Escape') {
        this.close();
      }
   },
   close(){
      this.$store.commit('logging_in', false);
   },
   stop(){
     
   }
  },
  created() {
    window.addEventListener('keyup', this.on_close);
  },
  destroyed() {
    window.removeEventListener('keyup', this.on_close);
  },

  computed: {
    ...mapGetters(["user"]),
    user_logged(){
      return this.user.jwt_payload && this.user.jwt_payload.exp > new Date().getTime() / 1000
    }
  },
};
</script>

<style scoped>
.login_background {
  display: flex;

  position: absolute;
  left: 0;
  top:0;
  width: 100vw;
  height: 100vh;

  background-color: rgba(0, 0, 0, 0.5);
}

.login {
  width: 300px;
  height: 210px;
  margin: auto; /* poussé de la moitié de hauteur de viewport */

  background: white;
  border-radius: 6px;
  border: solid #009999 3px;

  text-align: center;
  line-height: 60px;
  padding-top: 20px;
}

.inputs {
  line-height: 20px;
  width: 200px;
}

.submit.inputs {
  width: 208px;
}

</style>
