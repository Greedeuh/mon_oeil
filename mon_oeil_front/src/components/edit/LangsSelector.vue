<template>
  <div>
    <v-select multiple push-tags :options="know_langs" v-model="inner_langs" @input="$emit('change', $event)" />
  </div>
</template>

<script>
import vSelect from "vue-select";

const know_langs=["us", "fr"];

export default {
  name: "LangsSelector",
  components: { vSelect },
  model: {
    prop: 'langs',
    event: 'change'
  },
  props: {
    langs: Array
  },
  data: function(){
    return { inner_langs: this.langs, know_langs: know_langs}
  }
};
</script>

<style scoped>
#content {
  padding: 20px;
}
</style>
