<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import Titlebar from "./components/Titlebar.vue";
import Sidebar from "./components/Sidebar.vue";
import Viewer from "./components/Viewer.vue";
import {ref, reactive} from "vue";
import {appWindow} from "@tauri-apps/api/window"
import {convertFileSrc, invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import { open } from '@tauri-apps/api/dialog';

/*
import parser from 'woff2-parser';
*/
let fileName = ref("Undefined")
let dirFiles = ref([])
appWindow.onFileDropEvent((ev) => {
    if (ev.payload.type !== 'drop') {
        return
    }

    const [filepath] = ev.payload.paths// as string[]
    console.log(filepath)
    requestLoadFont(filepath)
})

let nameRecords = ref([])

requestListener()

async function requestListener() {
    await listen('request_detected', event => {
        console.log(`request_detected ${event.payload}`)
        ui.isOpenFile = true

    });
}

async function instanceDetectionLister() {
    await listen('init', event => {
        console.log(`init ${event.payload}`)
        alert(event.payload
        )
        addFontFace(event.payload)

    });
    await listen('file_request', event => {
        console.log(event.payload)
        addFontFace(event.payload)
    })
}

instanceDetectionLister()
let loadedFontFace;

function requestLoadFont(path) {
    addFontFace(path)
    invoke('request_name_data',
        {path: path})
        .then(data => {
            console.log('command_with_messge', data)
            dirFiles.value = data.dir_files
            const records = {}
            data.names.forEach((item, index)=> {
            })
            nameRecords.value = data.names
            /*
                fileName=message
            */
        })
}


invoke('request_name_data',
    {path: "C:\\Users\\ym174\\Desktop\\LINE_Seed_JP\\LINE_Seed_JP\\Web\\WOFF\\LINESeedJP_OTF_Bd.woff"})
    .then(message => {
        console.log('command_with_messge', message)
        dirFiles.value = message.dir_files
        /*
            fileName=message
        */
    })

function addFontFace(path) {
    loadedFontFace = new FontFace("LoadedFont", "url(" + convertFileSrc(path) + ")")
    loadedFontFace.load().then(function (loaded_face) {
        document.fonts.add(loaded_face);
        fileName.value = path
    }).catch(function (e) {
        alert('Failed to load the file: ' + path)
    });
    uiState.isOpenFile = true
}
async function requestFileDialog() {
    const selected = await open({
        multiple: false,
        filters: [{
            name: 'FontFile',
            extensions: ['ttf', 'otf', 'woff', 'woff2']
        }]
    })
    console.log(selected)
    requestLoadFont(selected)
}

let uiState = reactive({
    isOpenFile: false
})


</script>
<template>
    <div id="app-root">
        <Titlebar></Titlebar>
        <div v-if="!uiState.isOpenFile" class="blank_view">
            <p id="blank_message">Drop font file here or <button @click="requestFileDialog" class="small">Browse</button></p>
        </div>
        <div v-if="uiState.isOpenFile" class="container" id="main">
            <Sidebar class="sidebar" :dir-file-list="dirFiles"></Sidebar>
            <Viewer class="viewer" :filepath="fileName" :nameRecords="nameRecords"></Viewer>
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
    user-select: none;

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
