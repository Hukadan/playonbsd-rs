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
	<p v-if="game.engine">Engine: {{ game.engine }}</p>
	<p v-if="game.setup">Setup: {{ game.setup }}</p>
	<p v-if="game.runtime > 0">Runtime: {{ game.runtime }}</p>
        <p v-if="game.store" v-for="store in game.store">
	     <a :href="`${store}`">{{ store }}</a>
	</p>
	<p v-if="game.genres">
          <span v-for="genre in game.genres" class="label">{{ genre }}</span>
	</p>
      </div>
</template>

