<template>
  <div id="home">
    <Viewer id="viewer" :class="{ editor_mode: editor_mode }" />
    <Editor v-if="editor_mode" id="editor" />
  </div>
</template>

<script>
import { mapGetters } from 'vuex'

import Viewer from "../components/Viewer.vue";
import Editor from "../components/Editor.vue";

export default {
  name: "Home",
  components: {
    Viewer,
    Editor
  },
  computed: {
    ...mapGetters(['editor_mode'])
  },
};
</script>

<style scoped>
#home {
  display: flex;
}

#editor_mode {
  position: fixed;
}

#viewer.editor_mode {
  width: 50vw;
}

#editor {
  width: 50vw;
}
</style>
