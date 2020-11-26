<template>
  <div class="contribute">
    <span v-if="!user.jwt_payload">Vous devez être connecté pour contribuer. <a @click="login">Se connecter</a></span>
    <div class="contribution">
      <h1>Vos contributions</h1>
      <div class="gestures">
        <div v-for="(gesture, index) in gestures" :key="gesture.id">
          <Gesture
            @click.native="select(gesture)"
            class="gesture"
            :gesture="gesture"
          />
          <hr v-if="index != gestures.length - 1" />
        </div> 
      </div>
      <Pagination />
    </div>
    <Editor v-if="user.jwt_payload" class="editor" />
  </div>
</template>

<script>
import { mapGetters } from 'vuex'

import Editor from "../components/Editor.vue";
import Pagination from "../components/Pagination.vue"
import Gesture from "../components/gestures/Gesture.vue";

export default {
  name: "Contribute",
  components: {
    Editor,
    Pagination,
    Gesture
  },
  methods: {
    select(gesture) {
      this.$store.commit("select_gesture", gesture.id);
    },
    login() {
      this.$store.commit('logging_in', true);
    },
    on_login(mutation){
      if (this.contributions_gestures.length == 0
 && mutation.type === "jwt_payload" && this.user.jwt_payload) {
        this.load_contributions();
      }
    },
    load_contributions() {
        this.$store.commit('contribution_search', true);
        this.$store.commit('search_text', '');
        this.$store.dispatch('load_gestures');
    }
  },
  created() {
    if (!this.user.jwt_payload) {
      this.$store.commit('logging_in', true);
      this.$store.subscribe(this.on_login);
    } else {
      this.load_contributions();
    }
  },
  computed: {
    ...mapGetters(['gestures', 'user', 'search'])
  },
};
</script>

<style scoped>
.contribute {
  display: flex;
}
.contribution {
  width: 50%;
}

.editor {
  width: 50%;
}
</style>