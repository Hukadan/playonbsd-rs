<script setup>

const props = defineProps(['game'])

</script>

<template>
      <div v-if="game" >
        <p class="lead">{{ game.name }}</p>
	<p>
	<img v-if="game.cover" style="max-height: 250px" :src="`${game.cover}`" alt="">
	</p>
	<p v-if="game.year"><strong>Release date</strong>: {{ game.year.name }}</p>
	<p v-if="game.engine"><strong>Engine</strong>: {{ game.engine.name }}</p>
	<p v-if="game.setup"><strong>Setup</strong>: {{ game.setup }}</p>
	<p v-if="game.runtime"><strong>Runtime</strong>: {{ game.runtime.name }}</p>
	<p v-if="game.hints"><strong>Hints</strong>: {{ game.hints }}</p>
	<p v-if="game.dev"><strong>Developper</strong>: {{ game.dev.name }}</p>
	<p v-if="game.publi"><strong>Publisher</strong>: {{ game.publi.name }}</p>
	<p v-if="game.status"><strong>Status</strong>: {{ game.status }}</p>
	<div v-if="game.stores">
	  <p>Available here:</p>
	  <ul>
            <li v-for="store in game.stores">
	      <a :href="`${store}`">{{ store }}</a>
	    </li>
	  </ul>
	</div>
	<p v-if="game.genres">
          <span v-for="genre in game.genres" class="alert label">{{ genre.name }}</span>
	</p>
	<p v-if="game.tags">
          <span v-for="tag in game.tags" class="secondary label">{{ tag.name }}</span>
	</p>
      </div>
</template>

<style>
.label {
  margin: 0px 2px;
}
</style>

