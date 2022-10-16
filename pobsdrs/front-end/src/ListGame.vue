<script setup>
const props = defineProps(['games'])
</script>

<template>
      <div v-if="games" v-for="item in games.items">
        <dl>
          <dt @click="$emit('newid',item.id)">{{ item.name }}</dt>
          <dd>{{ item.year }}</dd>
          <dd>{{ item.engine }}</dd>
        </dl>
      </div>
</template>

<style>
.cell dl {
  cursor: pointer;
}
</style>
