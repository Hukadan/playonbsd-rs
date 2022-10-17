<script setup>
import { ref, watch } from 'vue'
import ListGame from './ListGame.vue' 
import DetailsGame from './DetailsGame.vue'
import TopBar from './TopBar.vue'

const gameData = ref(null)
const gameid = ref(1)
const searchtxt = ref("")
const allGames = ref(null)

const hostname = "127.0.0.1:8000"
//const hostname = "pobsdjs.chocolatines.org"

async function fetchGame() {
  const res = await fetch(
    `http://${hostname}/api/games/${gameid.value}`
  )
  gameData.value = await res.json()
}

async function fetchAllGames() {
  const res = await fetch(
    `http://${hostname}/api/games`
  )
  allGames.value = await res.json()
}

async function searchGamesByName() {
  const res = await fetch(
    `http://${hostname}/api/games?name=${searchtxt.value}`
  )
  allGames.value = await res.json()
}

fetchAllGames()
fetchGame()
watch(gameid, fetchGame)
watch(searchtxt, searchGamesByName)

</script>

<template>
    <div class="grid-x grid-padding-x align-center">
      <div class="cell small-10" style="max-height: 90vh; overflow-y :auto"  >
        <TopBar @newsearch="(search) => searchtxt = search" />
      </div>
    </div>
    <br>
    <div class="grid-x grid-padding-x align-center">
    <div class="cell small-4" style="max-height: 90vh; overflow-y :auto"  >
      <ListGame :games="allGames" @newid="(id) => gameid = id"/>
    </div>
    <div class="cell small-6" style="max-height: 90vh; overflow-y :auto"  >
      <DetailsGame :game="gameData" />
    </div>
    </div>
</template>

<style>
body {
  background: #17202A;
  color: white;
}
</style>
