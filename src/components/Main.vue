<template>
    <div id="config" class="modal hide">
        <button class="materialicon icon"  @click="control_modal('config')"  id="b_modalcontrol">close</button>
    </div>
    <div id="shade" class="hide"></div>

    <div id="container">
        <div id="toolbar">
            <p id="label_appname">Fontauri</p>
            <button id="b_setting" @click="control_modal('config')" class="materialicon icon">settings</button>


        </div>
        <div id="sidebar"><input type="text" id="path_field">
            <button id="path_loadbutton" class="materialicon">
                cloud_upload
            </button>
        </div>
        <div id="viewer">

            <p id="filename">{{ filename }}</p>
            <textarea class="preview">山路を登りながら、こう考えた。智に働けば角が立つ。</textarea>

            <p id="message">{{ message }}</p>
            <p id="path">{{ args }}</p>
        </div>
    </div>

</template>

<script>
import {convertFileSrc} from '@tauri-apps/api/tauri';
import {emit, listen} from '@tauri-apps/api/event'

export default {
    name: "Main.vue",
    data() {
        return {
            filename: 'ssss',
            isinput: true,
            args: [],
            fontt: null,
            message: 'なんか入力して',


        }
    },
    mounted: function () {
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


                }).catch(function(e){
                    alert(e)
            })// 戻り値を表示
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

            if(modal=='config'){
                let modal_config = document.getElementById("config");
                if(modal_config.classList.contains('hide')){
                    modal_config.classList.remove('hide')
                    shade.classList.remove('hide')

                }else{
                    modal_config.classList.add('hide')
                    shade.classList.add('hide')

                }
            }else{
                alert('No such a modal')
            }
        }

    }

}
</script>

<style scoped>

@import "stylesheets/default.css";

#b_setting {
    grid-column: 4/5;
    grid-row: 1/2;
}


#b_modalcontrol {
    position: absolute;
    top: 0;
    right: 7px;
}

#b_modalcontrol:hover{
    font-variation-settings: 'FILL' 0,
    'wght' 900,
    'GRAD' 0,
    'opsz' 48

}

#label_appname {
    color: #3d54ff;
    font-weight: 700;
    font-size: 1.5em;
    margin-left: 20px;
    line-height: 50px;
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
    background: #fff;
    border-radius: 8px;
    box-shadow: #989898 0px 0px 6px;
    z-index: 20;
    border: solid 1px #000;
    display: grid;
    grid-template-rows: 100%;
    grid-template-columns: 1fr 50px 50px 50px;
    /*
    margin: -10px -20px 0 -20px;
    */
}

#viewer {
    grid-column: 2/3;
    grid-row: 2/3;
    background: #e1e1e1;
    border: solid 1px #969696;
    padding: 20px 20px 20px 40px;
    border-radius: 8px;

}

#sidebar {
    grid-column: 1/2;
    grid-row: 2/3;
}

#container {
    display: grid;
    grid-template-columns: 200px 1fr;
    grid-template-rows: 50px 1fr;
    gap: 7px;
    height: calc(100vh - 6px * 2);
    /*
    height: 100vh;
    */
    padding: 7px;
    background: #dcdcdc;
    border-radius: 10px;
    box-shadow: black 0px 0px 6px;

}

#path_loadbutton {
}

.materialicon {
    font-family: 'material_icon';
}
</style>