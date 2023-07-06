<script setup>
import {appWindow, LogicalPosition, LogicalSize} from '@tauri-apps/api/window'
import {ref, reactive} from "vue";
import {onMounted} from '@vue/runtime-core'

let windowData = reactive({
    isMaximized: false
})
onMounted(() => {
    appWindow.isMaximized().then((isMaximized) => {
        if (isMaximized) {
            console.log('maximized(When app has launched)')
            windowData.isMaximized = true
        }
    })

    function setMaximizeIcon() {
        appWindow.isMaximized().then((isMaximized) => {
            if (isMaximized) {
                /*if (vm.ui.pinned) {
                    this.$refs.container.classList.remove("is_pinned")
                    appWindow.setAlwaysOnTop(false);
                    vm.ui.pinned = false
                }*/
                //console.log('maximized')
                windowData.isMaximized = true
            } else {
                //console.log('unMaximized')
                windowData.isMaximized = false
            }
        })

    }

    setInterval(setMaximizeIcon, 500)
})
const closeWindow = () => appWindow.close()
const minimizeWindow = () => appWindow.minimize()
const toggleWindowMaximize = (() => {
    windowData.isMaximized = !windowData.isMaximized
    appWindow.toggleMaximize()
})
</script>

<template>
    <div id="content">
        <!--        <button id="b_navigation">
                    <svg width="20" height="20" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M2.753 18h18.5a.75.75 0 0 1 .102 1.493l-.102.007h-18.5a.75.75 0 0 1-.102-1.493L2.753 18h18.5-18.5Zm0-6.497h18.5a.75.75 0 0 1 .102 1.493l-.102.007h-18.5a.75.75 0 0 1-.102-1.493l.102-.007h18.5-18.5Zm-.001-6.5h18.5a.75.75 0 0 1 .102 1.493l-.102.007h-18.5A.75.75 0 0 1 2.65 5.01l.102-.007h18.5-18.5Z" fill="#000000"/></svg>
                </button>-->
        <p data-tauri-drag-region @click="toggle_windowMaximize($event)" id="label_appname"><img
            class="logo_main"
            src="../assets/logo/logo_text_grey.svg">
        </p>
        <button id="b_close" @click="closeWindow()" class="icon win_control">
            <svg width="14" height="14" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path
                    d="m4.397 4.554.073-.084a.75.75 0 0 1 .976-.073l.084.073L12 10.939l6.47-6.47a.75.75 0 1 1 1.06 1.061L13.061 12l6.47 6.47a.75.75 0 0 1 .072.976l-.073.084a.75.75 0 0 1-.976.073l-.084-.073L12 13.061l-6.47 6.47a.75.75 0 0 1-1.06-1.061L10.939 12l-6.47-6.47a.75.75 0 0 1-.072-.976l.073-.084-.073.084Z"
                    fill="#212121"/>
            </svg>
        </button>
        <button id="b_maximize" @click="toggleWindowMaximize($event)" class="icon win_control">
            <svg v-if="!windowData.isMaximized" width="14" height="14" fill="none" viewBox="0 0 24 24"
                 xmlns="http://www.w3.org/2000/svg">
                <path
                    d="M5.75 3h12.5A2.75 2.75 0 0 1 21 5.75v12.5A2.75 2.75 0 0 1 18.25 21H5.75A2.75 2.75 0 0 1 3 18.25V5.75A2.75 2.75 0 0 1 5.75 3Zm0 1.5c-.69 0-1.25.56-1.25 1.25v12.5c0 .69.56 1.25 1.25 1.25h12.5c.69 0 1.25-.56 1.25-1.25V5.75c0-.69-.56-1.25-1.25-1.25H5.75Z"
                    fill="#212121"/>
            </svg>
            <svg v-if="windowData.isMaximized" width="14" height="14" fill="none" viewBox="0 0 24 24"
                 xmlns="http://www.w3.org/2000/svg">
                <path
                    d="M7.518 5H6.009a3.25 3.25 0 0 1 3.24-3h8.001A4.75 4.75 0 0 1 22 6.75v8a3.25 3.25 0 0 1-3 3.24v-1.508a1.75 1.75 0 0 0 1.5-1.732v-8a3.25 3.25 0 0 0-3.25-3.25h-8A1.75 1.75 0 0 0 7.518 5ZM5.25 6A3.25 3.25 0 0 0 2 9.25v9.5A3.25 3.25 0 0 0 5.25 22h9.5A3.25 3.25 0 0 0 18 18.75v-9.5A3.25 3.25 0 0 0 14.75 6h-9.5ZM3.5 9.25c0-.966.784-1.75 1.75-1.75h9.5c.967 0 1.75.784 1.75 1.75v9.5a1.75 1.75 0 0 1-1.75 1.75h-9.5a1.75 1.75 0 0 1-1.75-1.75v-9.5Z"
                    fill="#212121"/>
            </svg>
        </button>

        <button id="b_minimize" @click="minimizeWindow()" class="icon win_control">
            <svg width="14" height="14" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path d="M3.755 12.5h16.492a.75.75 0 0 0 0-1.5H3.755a.75.75 0 0 0 0 1.5Z" fill="#212121"/>
            </svg>
        </button>

        <button id="b_setting" @click="control_modal('config')" class="materialicon icon">
            <svg width="20" height="20" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path
                    d="M12.012 2.25c.734.008 1.465.093 2.182.253a.75.75 0 0 1 .582.649l.17 1.527a1.384 1.384 0 0 0 1.927 1.116l1.401-.615a.75.75 0 0 1 .85.174 9.792 9.792 0 0 1 2.204 3.792.75.75 0 0 1-.271.825l-1.242.916a1.381 1.381 0 0 0 0 2.226l1.243.915a.75.75 0 0 1 .272.826 9.797 9.797 0 0 1-2.204 3.792.75.75 0 0 1-.848.175l-1.407-.617a1.38 1.38 0 0 0-1.926 1.114l-.169 1.526a.75.75 0 0 1-.572.647 9.518 9.518 0 0 1-4.406 0 .75.75 0 0 1-.572-.647l-.168-1.524a1.382 1.382 0 0 0-1.926-1.11l-1.406.616a.75.75 0 0 1-.849-.175 9.798 9.798 0 0 1-2.204-3.796.75.75 0 0 1 .272-.826l1.243-.916a1.38 1.38 0 0 0 0-2.226l-1.243-.914a.75.75 0 0 1-.271-.826 9.793 9.793 0 0 1 2.204-3.792.75.75 0 0 1 .85-.174l1.4.615a1.387 1.387 0 0 0 1.93-1.118l.17-1.526a.75.75 0 0 1 .583-.65c.717-.159 1.45-.243 2.201-.252Zm0 1.5a9.135 9.135 0 0 0-1.354.117l-.109.977A2.886 2.886 0 0 1 6.525 7.17l-.898-.394a8.293 8.293 0 0 0-1.348 2.317l.798.587a2.881 2.881 0 0 1 0 4.643l-.799.588c.32.842.776 1.626 1.348 2.322l.905-.397a2.882 2.882 0 0 1 4.017 2.318l.11.984c.889.15 1.798.15 2.687 0l.11-.984a2.881 2.881 0 0 1 4.018-2.322l.905.396a8.296 8.296 0 0 0 1.347-2.318l-.798-.588a2.881 2.881 0 0 1 0-4.643l.796-.587a8.293 8.293 0 0 0-1.348-2.317l-.896.393a2.884 2.884 0 0 1-4.023-2.324l-.11-.976a8.988 8.988 0 0 0-1.333-.117ZM12 8.25a3.75 3.75 0 1 1 0 7.5 3.75 3.75 0 0 1 0-7.5Zm0 1.5a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5Z"
                    fill="#212121"/>
            </svg>
        </button>

        <button id="b_pin" @click="pinWindow()" class="materialicon icon">
            <svg width="20" height="20" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path
                    d="m16.242 2.932 4.826 4.826a2.75 2.75 0 0 1-.715 4.404l-4.87 2.435a.75.75 0 0 0-.374.426l-1.44 4.166a1.25 1.25 0 0 1-2.065.476L8.5 16.561 4.06 21H3v-1.06l4.44-4.44-3.105-3.104a1.25 1.25 0 0 1 .476-2.066l4.166-1.44a.75.75 0 0 0 .426-.373l2.435-4.87a2.75 2.75 0 0 1 4.405-.715Zm3.766 5.886-4.826-4.826a1.25 1.25 0 0 0-2.002.325l-2.435 4.871a2.25 2.25 0 0 1-1.278 1.12l-3.789 1.31 6.705 6.704 1.308-3.789a2.25 2.25 0 0 1 1.12-1.277l4.872-2.436a1.25 1.25 0 0 0 .325-2.002Z"
                    fill="#212121"/>
            </svg>
        </button>
        <button id="b_pin" @click="pinWindow()" class="materialicon icon filled">
            <svg width="20" height="20" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path
                    d="m21.068 7.758-4.826-4.826a2.75 2.75 0 0 0-4.404.715l-2.435 4.87a.75.75 0 0 1-.426.374L4.81 10.33a1.25 1.25 0 0 0-.476 2.065L7.439 15.5 3 19.94V21h1.06l4.44-4.44 3.104 3.105a1.25 1.25 0 0 0 2.066-.476l1.44-4.166a.75.75 0 0 1 .373-.426l4.87-2.435a2.75 2.75 0 0 0 .715-4.404Z"
                    fill="#3d54ff"/>
            </svg>
        </button>
    </div>
</template>
<style scoped>
.logo_main {
    height: 40%;
}

.win_control > svg {
    margin: auto auto;
}

#b_minimize:hover, #b_maximize:hover {
    background: #d3dce5;
    border-radius: 0;

}
#b_minimize {
    grid-column: 4/5;
    grid-row: 1/2;
    /*
    transform:translateY(-6px);
    */

}
#b_maximize {
    grid-column: 5/6;
    grid-row: 1/2;
    /*
    transform:translateY(-6px);
    */

}

#content {
    background: #e0ecf8;
    height: 40px;
    /*    grid-column: 2/4;
        grid-row: 1/2;*/
    /*
    background: #fff;
    */
    margin: 0px -0px;
    border-radius: 7px 7px 0px 0px;
    /*
    border-radius: 8px;
    */
    /*
    box-shadow: #989898 0px 0px 6px;
    */
    z-index: 20;
    /*
    border: solid 1px #000;
    */
    display: grid;
    grid-template-rows: 100%;
    grid-template-columns: 1fr 40px 40px 47px 47px 47px;
    /*
    margin: -10px -20px 0 -20px;
    */
}

button {
    background: none;
    border: none;
}

#label_appname {
    color: #3d54ff;
    font-weight: 700;
    font-size: 1.25em;
    margin-left: 10px;
    line-height: 43px;
    grid-column: 1/2;
    grid-row: 1/2;
}

#b_close {
    grid-column: 6/7;
    grid-row: 1/2;
}

#b_close:hover {
    background: #ff2424;
    border-radius: 0px;
}

#b_close:hover path {
    fill: #fff;
    border-radius: 0px;
}

#b_navigation {
    margin-left: 5px;
    padding: 10px;
}

#b_pin {
    grid-column: 3/4;
    grid-row: 1/2;
}


#b_setting {
    grid-column: 2/3;
    grid-row: 1/2;
}
</style>