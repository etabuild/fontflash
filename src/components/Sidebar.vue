<script setup>
import {ref, reactive} from "vue";

const props = defineProps({
    dirFileList: Array
})
let ui = reactive({
    isOpenFile: false,
    isOpenSidebar: false
})

function openSidebar(e) {
    ui.isOpenSidebar = !ui.isOpenSidebar
    if (ui.isOpenSidebar) {
        e.target.closest(".container").classList.add("sidebar-open")
        e.target.closest(".sidebar_icon").classList.add("isopen")
    } else {
        e.target.closest(".container").classList.remove("sidebar-open")
        e.target.closest(".sidebar_icon").classList.remove("isopen")
    }
}

function requestLoadDir(){

}
</script>

<template>
    <div id="sidebar">
        <div id="sidebar-icons">
            <button class="sidebar_icon" @click="openSidebar($event)">
                <svg width="24" height="24" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M8.207 4c.46 0 .908.141 1.284.402l.156.12L12.022 6.5h7.728a2.25 2.25 0 0 1 2.229 1.938l.016.158.005.154v9a2.25 2.25 0 0 1-2.096 2.245L19.75 20H4.25a2.25 2.25 0 0 1-2.245-2.096L2 17.75V6.25a2.25 2.25 0 0 1 2.096-2.245L4.25 4h3.957Zm1.44 5.979a2.25 2.25 0 0 1-1.244.512l-.196.009-4.707-.001v7.251c0 .38.282.694.648.743l.102.007h15.5a.75.75 0 0 0 .743-.648l.007-.102v-9a.75.75 0 0 0-.648-.743L19.75 8h-7.729L9.647 9.979ZM8.207 5.5H4.25a.75.75 0 0 0-.743.648L3.5 6.25v2.749L8.207 9a.75.75 0 0 0 .395-.113l.085-.06 1.891-1.578-1.89-1.575a.75.75 0 0 0-.377-.167L8.207 5.5Z"
                        fill="#212121"/>
                </svg>
                <!--
                                <svg width="24" height="24" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M13.821 6.5h5.929a2.25 2.25 0 0 1 2.229 1.938l.016.158.005.154v9a2.25 2.25 0 0 1-2.096 2.245L19.75 20H4.25a2.25 2.25 0 0 1-2.245-2.096L2 17.75v-7.251l6.207.001.196-.009a2.25 2.25 0 0 0 1.088-.393l.156-.12L13.821 6.5ZM8.207 4c.46 0 .908.141 1.284.402l.156.12 2.103 1.751-3.063 2.553-.085.061a.75.75 0 0 1-.29.106L8.206 9 2 8.999V6.25a2.25 2.25 0 0 1 2.096-2.245L4.25 4h3.957Z" fill="#212121"/></svg>
                -->
            </button>
        </div>


    </div>
    <div id="sidebar-content" v-if="ui.isOpenSidebar">
        <div class="content" v-if="true">
            <div id="folder-viewer-control">
                <button @click="loadDirFiles">
                    cloud_upload
                </button>
                <div class="item-file" v-for="(filename, index) in props.dirFileList">
                    <img class="file-icon" src="../assets/icons/fileicon/fileicon_ttf.svg">

                    <p class="label-filename">{{filename}}</p>
                </div>
            </div>

            <!--                <div id="folder-viewer">
                                <div class="dir-file" v-for="(filename, index) in dir.fontFileList" @click="dirFileOnClick($event,index)">
                                    <img class="file-icon" src="../assets/icons/fileicon/fileicon_ttf.svg">
                                    <p class="label-filename">{{ filename }}</p>
                                </div>
                            </div>-->
        </div>
    </div>

</template>

<style scoped>
#sidebar-content {
    grid-column: 2/3;
    grid-row: 1/2;
    overflow: hidden;
    border-radius: 8px;
    margin-bottom: 12px;
    margin-right: 12px;
    border:1px solid #707070;
    background-color: #ffffff
}

.sidebar_icon {
    padding: 4px;
    color: #ffffff;
    width: 32px;
    height: 32px;
    margin: 4px 9px;
    line-height: 32px;
    border-radius: 4px;
    background: none;
}

.sidebar_icon.isopen{
    background: #728eab;

}
.sidebar_icon.isopen svg path{
    fill: #fff;
}
.sidebar_icon.isopen:hover {
    background: #728eab;

}

.sidebar_icon:hover {
    background: #e0ecf8;
    font-variation-settings: 'FILL' 0
}

#sidebar-content {
    grid-column: 2/3;
    grid-row: 2/3;
    overflow-y: scroll;
    background-color: #ffffff;
    padding: 5px;

}

#sidebar-content::-webkit-scrollbar-thumb{
}

#sidebar:hover {
/*    border: solid 1px #8a8a8a;
    background: #ffffff;*/

    margin: 0px;
}


#sidebar {
    border: none;
    /*
    margin: 1px;
    */
    background: none;
    display: grid;
    grid-column: 1/2;
    grid-row: 2/3;
    /*
    border: solid 1px #969696;
    */
    border-radius: 8px;
    height: 40px;
}
.item-file {
    height: auto;
    /*
    background: #a1a1a1;
    */
    margin-right: 0px;
    margin-bottom: 5px;
    padding: 5px;
    border-radius: 3px;
}

.item-file:hover {
    background: #eeeeee;
}
.label-filename {
    font-size: 0.8em;
    height: 50%;
    word-break: break-all;
    overflow: hidden;
    margin-top: -4px;
}
.file-icon {
    width: 40px;
    display: inline-block;
}
</style>