<script setup>
const props = defineProps(['games'])
</script>

<template>
      <div v-if="games" v-for="item in games.items">
        <dl>
          <dt @click="$emit('new_game',item.self)">{{ item.name }}</dt>
          <dd v-if="item.year">{{ item.year.name }}</dd>
          <dd v-if="item.engine">{{ item.engine.name }}</dd>
        </dl>
      </div>
</template>

<style>
.cell dl {
  cursor: pointer;
}
</style>
