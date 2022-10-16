<script setup>

const props = defineProps(['game'])

</script>

<template>
      <div v-if="game" >
        <p class="lead">{{ game.name }}</p>
	<p>
	<img style="max-height: 250px" :src="`${game.cover}`" alt="">
	</p>
	<p>Release date: {{ game.year }}</p>
	<p v-if="game.engine.length > 0">Engine: {{ game.engine }}</p>
	<p v-if="game.engine.setup > 0">Setup: {{ game.setup }}</p>
	<p v-if="game.engine.runtime > 0">Runtime: {{ game.runtime }}</p>
        <p v-for="store in game.store">
	     <a :href="`${store}`">{{ store }}</a>
	</p>
	<p>
          <span v-for="genre in game.genres" class="label">{{ genre }}</span>
	</p>
      </div>
</template>

