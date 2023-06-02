<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Main from "./components/Main.vue";
import Titlebar from "./components/Titlebar.vue";
import Sidebar from "./components/Sidebar.vue";
import Viewer from "./components/Viewer.vue";
import {ref, reactive} from "vue";

let uiState = reactive({
    isOpenFile: true
})
</script>
<template>
    <div id="app-root">
        <Titlebar class="titlebar"></Titlebar>

        <div v-if="!uiState.isOpenFile" class="blank_view">
            <p id="blank_message">ファイルを開いてね☆</p>
        </div>
        <div v-if="uiState.isOpenFile" class="container" id="main">
            <Sidebar class="sidebar"></Sidebar>
            <Viewer class="viewer"></Viewer>
        </div>
    </div>
</template>

<style>
@import 'assets/fonts/fonts.css';
@import 'assets/icons/icons.css';
@import "components/stylesheets/reset.css";
@import 'layout.css';

*, body {
    font-family: notosans, 'BIZ UDPゴシック',sans-serif;
    margin: 0;
    padding: 0;
}

html, body {
    border-radius: 30px;
    opacity: 1;
    background: none;
}

body {
    overflow: hidden;
    min-width: 500px;

}

.titlebar {
}


</style>
