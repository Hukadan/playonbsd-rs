<script setup>

const props = defineProps(['game'])

</script>

<template>
      <div v-if="game" >
        <p class="lead">{{ game.name }}</p>
	<p>
	<img v-if="game.cover" style="max-height: 250px" :src="`${game.cover}`" alt="">
	</p>
	<p v-if="game.year"><strong>Release date</strong>: {{ game.year }}</p>
	<p v-if="game.engine"><strong>Engine</strong>: {{ game.engine }}</p>
	<p v-if="game.setup"><strong>Setup</strong>: {{ game.setup }}</p>
	<p v-if="game.runtime"><strong>Runtime</strong>: {{ game.runtime }}</p>
	<p v-if="game.hints"><strong>Hints</strong>: {{ game.hints }}</p>
	<p v-if="game.dev"><strong>Developper</strong>: {{ game.dev }}</p>
	<p v-if="game.publi"><strong>Publisher</strong>: {{ game.publi }}</p>
	<p v-if="game.status"><strong>Status</strong>: {{ game.status }}</p>
	<div v-if="game.store">
	  <p>Available here:</p>
	  <ul>
            <li v-if="game.store" v-for="store in game.store">
	      <a :href="`${store}`">{{ store }}</a>
	    </li>
	  </ul>
	</div>
	<p v-if="game.genres">
          <span v-for="genre in game.genres" class="alert label">{{ genre }}</span>
	</p>
	<p v-if="game.tags">
          <span v-for="tag in game.tags" class="secondary label">{{ tag }}</span>
	</p>
      </div>
</template>

<style>
.label {
  margin: 0px 2px;
}
</style>

