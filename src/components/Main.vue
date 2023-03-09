<template>
    <div id="config" class="modal hide">
        <button class="materialicon icon" @click="control_modal('config')" id="b_modalcontrol">close</button>
    </div>
    <div data-tauri-drag-region id="shade" class="hide"></div>

    <div id="container" class="is_pinned" @select="block_select">

        <div id="toolbar">
            <p data-tauri-drag-region @click="toggle_windowMaximize($event)" id="label_appname">FontFlash</p>
            <button id="b_close" @click="close_window()" class="icon win_control">
                <svg width="14" height="14" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="m4.397 4.554.073-.084a.75.75 0 0 1 .976-.073l.084.073L12 10.939l6.47-6.47a.75.75 0 1 1 1.06 1.061L13.061 12l6.47 6.47a.75.75 0 0 1 .072.976l-.073.084a.75.75 0 0 1-.976.073l-.084-.073L12 13.061l-6.47 6.47a.75.75 0 0 1-1.06-1.061L10.939 12l-6.47-6.47a.75.75 0 0 1-.072-.976l.073-.084-.073.084Z"
                        fill="#212121"/>
                </svg>
            </button>
            <button id="b_maximize" @click="toggle_windowMaximize($event)" class="icon win_control">
                <svg v-if="!maximized" width="14" height="14" fill="none" viewBox="0 0 24 24"
                     xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M5.75 3h12.5A2.75 2.75 0 0 1 21 5.75v12.5A2.75 2.75 0 0 1 18.25 21H5.75A2.75 2.75 0 0 1 3 18.25V5.75A2.75 2.75 0 0 1 5.75 3Zm0 1.5c-.69 0-1.25.56-1.25 1.25v12.5c0 .69.56 1.25 1.25 1.25h12.5c.69 0 1.25-.56 1.25-1.25V5.75c0-.69-.56-1.25-1.25-1.25H5.75Z"
                        fill="#212121"/>
                </svg>
                <svg v-if="maximized" width="14" height="14" fill="none" viewBox="0 0 24 24"
                     xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M7.518 5H6.009a3.25 3.25 0 0 1 3.24-3h8.001A4.75 4.75 0 0 1 22 6.75v8a3.25 3.25 0 0 1-3 3.24v-1.508a1.75 1.75 0 0 0 1.5-1.732v-8a3.25 3.25 0 0 0-3.25-3.25h-8A1.75 1.75 0 0 0 7.518 5ZM5.25 6A3.25 3.25 0 0 0 2 9.25v9.5A3.25 3.25 0 0 0 5.25 22h9.5A3.25 3.25 0 0 0 18 18.75v-9.5A3.25 3.25 0 0 0 14.75 6h-9.5ZM3.5 9.25c0-.966.784-1.75 1.75-1.75h9.5c.967 0 1.75.784 1.75 1.75v9.5a1.75 1.75 0 0 1-1.75 1.75h-9.5a1.75 1.75 0 0 1-1.75-1.75v-9.5Z"
                        fill="#212121"/>
                </svg>
            </button>

            <button id="b_minimize" @click="minimize_window()" class="icon win_control">
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

            <button id="b_pin" v-if="pinned===false" @click="pinWindow()" class="materialicon icon">
                <svg width="20" height="20" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="m16.242 2.932 4.826 4.826a2.75 2.75 0 0 1-.715 4.404l-4.87 2.435a.75.75 0 0 0-.374.426l-1.44 4.166a1.25 1.25 0 0 1-2.065.476L8.5 16.561 4.06 21H3v-1.06l4.44-4.44-3.105-3.104a1.25 1.25 0 0 1 .476-2.066l4.166-1.44a.75.75 0 0 0 .426-.373l2.435-4.87a2.75 2.75 0 0 1 4.405-.715Zm3.766 5.886-4.826-4.826a1.25 1.25 0 0 0-2.002.325l-2.435 4.871a2.25 2.25 0 0 1-1.278 1.12l-3.789 1.31 6.705 6.704 1.308-3.789a2.25 2.25 0 0 1 1.12-1.277l4.872-2.436a1.25 1.25 0 0 0 .325-2.002Z"
                        fill="#212121"/>
                </svg>
            </button>
            <button id="b_pin" v-if="pinned===true" @click="pinWindow()" class="materialicon icon filled">
                <svg width="20" height="20" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="m21.068 7.758-4.826-4.826a2.75 2.75 0 0 0-4.404.715l-2.435 4.87a.75.75 0 0 1-.426.374L4.81 10.33a1.25 1.25 0 0 0-.476 2.065L7.439 15.5 3 19.94V21h1.06l4.44-4.44 3.104 3.105a1.25 1.25 0 0 0 2.066-.476l1.44-4.166a.75.75 0 0 1 .373-.426l4.87-2.435a2.75 2.75 0 0 0 .715-4.404Z"
                        fill="#212121"/>
                </svg>
            </button>

        </div>
        <div id="sidebar">
            <button class="materialicon icon sidebar_icon" @click="openSidebar($event)">
                <svg width="24" height="24" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path
                        d="M8.207 4c.46 0 .908.141 1.284.402l.156.12L12.022 6.5h7.728a2.25 2.25 0 0 1 2.229 1.938l.016.158.005.154v9a2.25 2.25 0 0 1-2.096 2.245L19.75 20H4.25a2.25 2.25 0 0 1-2.245-2.096L2 17.75V6.25a2.25 2.25 0 0 1 2.096-2.245L4.25 4h3.957Zm1.44 5.979a2.25 2.25 0 0 1-1.244.512l-.196.009-4.707-.001v7.251c0 .38.282.694.648.743l.102.007h15.5a.75.75 0 0 0 .743-.648l.007-.102v-9a.75.75 0 0 0-.648-.743L19.75 8h-7.729L9.647 9.979ZM8.207 5.5H4.25a.75.75 0 0 0-.743.648L3.5 6.25v2.749L8.207 9a.75.75 0 0 0 .395-.113l.085-.06 1.891-1.578-1.89-1.575a.75.75 0 0 0-.377-.167L8.207 5.5Z"
                        fill="#212121"/>
                </svg>
                <!--
                                <svg width="24" height="24" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M13.821 6.5h5.929a2.25 2.25 0 0 1 2.229 1.938l.016.158.005.154v9a2.25 2.25 0 0 1-2.096 2.245L19.75 20H4.25a2.25 2.25 0 0 1-2.245-2.096L2 17.75v-7.251l6.207.001.196-.009a2.25 2.25 0 0 0 1.088-.393l.156-.12L13.821 6.5ZM8.207 4c.46 0 .908.141 1.284.402l.156.12 2.103 1.751-3.063 2.553-.085.061a.75.75 0 0 1-.29.106L8.206 9 2 8.999V6.25a2.25 2.25 0 0 1 2.096-2.245L4.25 4h3.957Z" fill="#212121"/></svg>
                -->
            </button>
            <div class="content" v-if="isOpenSidebar">
                <input type="text" id="path_field">
                <button id="path_loadbutton" class="materialicon">
                    cloud_upload
                </button>
            </div>
        </div>
        <div id="viewer">

            <p id="filename">{{ filename }}</p>
            <textarea @input="set_preview_height"
                      class="preview">あのイーハトーヴォのすきとおった風、夏でも底にrrrrr冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。</textarea>

            <p id="message">{{ message }}</p>
            <p id="path">{{ args }}</p>
        </div>
    </div>

</template>

<script>
import {convertFileSrc} from '@tauri-apps/api/tauri';
import {emit, listen} from '@tauri-apps/api/event'
import {appWindow} from '@tauri-apps/api/window'

export default {
    name: "Main.vue",
    data() {
        return {
            filename: 'ssss',
            isinput: true,
            args: [],
            fontt: null,
            message: 'v0.1 tauri',
            pinned: false,
            fs: null,
            isOpenSidebar: false,
            maximized: false
        }
    },
    mounted: function () {
        appWindow.isMaximized().then((isMaximized) => {
            if (isMaximized) {
                this.$data.maximized = true
            }
        })
        /*        const path = process.cwd();
        // ファイル名の一覧
                const filenames = fs.readdirSync(convertFileSrc(path));
                console.log(filenames);*/
        console.log('aa');
        this.getArgs();
        let unlisten;
        var a = this
        var vm = this.$data

        async function f() {
            unlisten = await listen('instance_detection', event => {
                console.log(`instance_detection ${event.payload} ${new Date()}`)
                a.load_font(vm, event.payload.split(","))
            });
        }

        f()


    },
    methods: {
        getArgs: function () {
            this.$data.isinput = false;
            var invoke = window.__TAURI__.invoke

            console.log('!')
            var vm = this.$data
            var a = this
            invoke('get_args', {})
                .then(function (rustMsg) {

                    var args = rustMsg.split(",")
                    if (args[1] != null) {
                        console.log(args[1])
                    } else {
                        args[1] = "C:\\Users\\ym174\\OneDrive\\デスクトップ\\TBGoStdR-C6.otf"
                    }
                    a.load_font(vm, args)


                }).catch(function (e) {
                alert(e)
            })// 戻り値を表示

            invoke('get_data', {}).then(function (data) {
                console.log(data)
            }).catch(function (e) {
                alert(e)
            })
        },
        load_font: function (vm, args) {
            /*
                        document.fonts.delete(vm.fontt)
            */
            vm.fontt = new FontFace("LoadedFont", "url(" + convertFileSrc(args[1]) + ")")
            vm.fontt.load().then(function (loaded_face) {
                /// フォント読み込み成功
                /// body要素全体にそれを適用する
                document.fonts.add(loaded_face);
                var splited_path = args[1].split('\\')

                vm.filename = splited_path[splited_path.length - 1]
                /*
                                            document.body.style.fontFamily = '"Cosmos Logic"';
                */
            }).catch(function (e) {
                /// フォント読み込み失敗
                alert('Failed to load font');
                vm.message = '読み込めんかった';
            });
        },
        control_modal: function (modal) {
            let shade = document.getElementById("shade");

            if (modal == 'config') {
                let modal_config = document.getElementById("config");
                if (modal_config.classList.contains('hide')) {
                    modal_config.classList.remove('hide')
                    shade.classList.remove('hide')

                } else {
                    modal_config.classList.add('hide')
                    shade.classList.add('hide')

                }
            } else {
                alert('No such a modal')
            }
        },
        close_window: function () {
            appWindow.close();
        },
        toggle_windowMaximize: function (event) {
            appWindow.toggleMaximize();
            this.$data.maximized = !this.$data.maximized
            alert(this.$data.maximized)

        },
        minimize_window: function () {
            appWindow.minimize();
        },
        set_preview_height: function () {
            this.style.height = "auto";
            this.style.height = `${this.scrollHeight}px`;
        },
        pinWindow: function () {
            if (this.$data.pinned == false) {
                appWindow.setAlwaysOnTop(true);
                this.$data.pinned = true;


            } else {
                appWindow.setAlwaysOnTop(false);
                this.$data.pinned = false;
            }
        },
        openSidebar: function (event) {
            console.log(event)
            event.target.style.backgroundColor = "#7d8dff"
            event.target.style.color = "#fff"

            event.target.children[0].style.display = "block"

        }


    }

}
</script>

<style scoped>

@import "stylesheets/default.css";

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

#b_pin {
    grid-column: 3/4;
    grid-row: 1/2;
}


#b_setting {
    grid-column: 2/3;
    grid-row: 1/2;
}

/*#container.is_pinned{
    border: solid 4px #3d54ff;
    border-radius: 7px;
}*/

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

#b_minimize:hover, #b_maximize:hover {
    background: #d0d0d0;
    border-radius: 0;

}


#b_modalcontrol {
    position: absolute;
    top: 0;
    right: 7px;
}

#b_modalcontrol:hover {
    font-variation-settings: 'FILL' 0,
    'wght' 900,
    'GRAD' 0,
    'opsz' 48

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


.preview {
    border: none;
    resize: none;
    font-size: 2em;
    font-family: 'LoadedFont';
    margin: 5px;
    background: none;
    width: calc(100% - 10px);
    line-height: 1.2;
    overflow: hidden;
    background: #f3f3f3;
    border-radius: 10px;
    padding: 10px;
    height: 10em;

}

.preview :focus {
    outline: none;
    background: #000;
}

#filename {
    font-weight: 600;
    color: #000;
    font-size: 1.8em;
    margin-left: 2px;
}

#toolbar {
    grid-column: 1/3;
    grid-row: 1/2;
    /*
    background: #fff;
    */
    margin: 0px -12px;
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

#viewer {
    grid-column: 2/3;
    grid-row: 2/3;
    background: #eaeaea;
    /*
    border: solid 1px #969696;
    */
    padding: 20px 20px 20px 40px;
    border-radius: 0px 8px 8px 0px;
    overflow-y: scroll;
    overflow-x: hidden;

}

#sidebar {
    grid-column: 1/2;
    grid-row: 2/3;
    background: #eaeaea;
    /*
    border: solid 1px #969696;
    */
    border-radius: 8px 0px 0px 8px;

}

#container {
    display: grid;
    grid-template-columns: 40px 1fr;
    grid-template-rows: 40px 1fr;
    gap: 3px;
    height: 100vh;
    /*
    height: 100vh;
    */
    padding: 0px 12px 12px 12px;
    background: #dcdcdc;

    /*
    border-radius: 10px;
    */
    /*
    box-shadow: black 0px 0px 6px;
    */

}


#path_loadbutton {
}

.materialicon {
    font-family: 'material_icon';
}

.sidebar_icon {
    color: #707070;
    width: 100%;
    width: 32px;
    height: 32px;
    margin: 4px auto;
    line-height: 32px;
    border-radius: 4px;
}

.sidebar_icon:hover {
    background: #dcdcdc;
    font-variation-settings: 'FILL' 0
}

.materialicon.filled {
    font-variation-settings: 'FILL' 1,
    'wght' 900,
    'GRAD' 0,
    'opsz' 48
}
</style>