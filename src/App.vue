<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Main from "./components/Main.vue";
import Titlebar from "./components/Titlebar.vue";
import Sidebar from "./components/Sidebar.vue";
import Viewer from "./components/Viewer.vue";
import {ref, reactive} from "vue";
import {appWindow} from "@tauri-apps/api/window"
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
/*
import parser from 'woff2-parser';
*/

appWindow.onFileDropEvent((ev) => {
    if (ev.payload.type !== 'drop') {
        return
    }
    const [filepath] = ev.payload.paths// as string[]
    console.log(filepath)
    requestLoadFont(filepath)
    //=> /absolute/path/example.txt
})
async function instanceDetectionLister() {
    await listen('init', event => {
        console.log(`init ${event.payload}`)

    });
}
instanceDetectionLister()
let loadedFontFace;
function requestLoadFont(path){
    addFontFace(path)

}

function addFontFace(path) {
   loadedFontFace = new FontFace("LoadedFont", "url(" + convertFileSrc(path) + ")")
    loadedFontFace.load().then(function (loaded_face) {
        /// フォント読み込み成功
        /// body要素全体にそれを適用する
        document.fonts.add(loaded_face);
/*        if (path !== null) {
            let splited_path = fontdata.filepath.split('\\')
            vm.filename = splited_path[splited_path.length - 1]
        }*/


    }).catch(function (e) {
        /// フォント読み込み失敗
        alert('Failed to load the file: '+ path)
/*
        message = 'フォントの読み込みに失敗';
*/
    });
}


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
    font-family: notosans, 'BIZ UDPゴシック', sans-serif;
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
