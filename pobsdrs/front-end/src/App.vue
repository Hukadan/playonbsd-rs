<script setup>
import { ref, watch } from 'vue'
import ListGame from './components/ListGame.vue' 
import DetailsGame from './components/DetailsGame.vue'
import TopBar from './components/TopBar.vue'

const gameData = ref(null)
const gameid = ref(1)
const searchtxt = ref("")
const allGames = ref(null)
//const hostname = "127.0.0.1:8000"
//const hostname = ref("pobsdjs.chocolatines.org")
//const conf = ref("nul")

//async function fetchConfig() {
//  const res = await fetch("config.json")
//  conf.value = await res.json()
//}
//fetchConfig()


async function fetchGame() {
  const res = await fetch(
    //`http://${conf.value.hostname}/api/games/${gameid.value}`
    `api/games/${gameid.value}`
  )
  gameData.value = await res.json()
}

async function fetchAllGames() {
  const res = await fetch(
    //`http://${conf.hostname}/api/games`
    `api/games`
  )
  allGames.value = await res.json()
}

async function searchGamesByName() {
  const res = await fetch(
    //`http://${conf.hostname}/api/games/orsearch?name=${searchtxt.value}&engine=${searchtxt.value}&runtime=${searchtxt.value}&genre=${searchtxt.value}&tag=${searchtxt.value}&year=${searchtxt.value}&dev=${searchtxt.value}&publi=${searchtxt.value}`
    `api/games/orsearch?name=${searchtxt.value}&engine=${searchtxt.value}&runtime=${searchtxt.value}&genre=${searchtxt.value}&tag=${searchtxt.value}&year=${searchtxt.value}&dev=${searchtxt.value}&publi=${searchtxt.value}`

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
@import '@/assets/foundation.css';
body {
  background: #17202A;
  color: #a2a2a2;
}
.menu-text {
color: black;
}
.top-bar, .top-bar ul {
background-color: #794242;
}
.top-bar a {
color: #0a2435;
}
input[type="search"] {
background-color: #6f5555;
}
</style>
