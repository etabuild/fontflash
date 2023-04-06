<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Main from "./components/Main.vue";

</script>
<template>
    <Main/>
</template>

<style>
@import 'assets/fonts/fonts.css';
@import 'assets/icons/icons.css';
@import "components/stylesheets/reset.css";
*,body{
   font-family: notosans, 'BIZ UDPゴシック';
    margin: 0;
    padding: 0;
}
html,body{
    border-radius: 30px;
    opacity: 1;
    background: none;
}
body{

    min-width: 500px;

}
</style>
