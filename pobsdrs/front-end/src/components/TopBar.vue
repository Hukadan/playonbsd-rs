<template>
<div class="top-bar">
  <div class="top-bar">
      <div class="menu-text align-right">
        PlayOnBSD JS
	-- see <a href="http://playonbsd.com">http://playonbsd.com</a> for the original
      </div>
  </div>
  <div class="top-bar-right">
    <ul class="menu">
      <li><input :value="search" @input="$emit('newsearch', $event.target.value)" type="search" placeholder="Search"></li>
    </ul>
  </div>
</div>
</template>
