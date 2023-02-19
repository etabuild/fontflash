<template>
    <div id="toolbar"></div>
    <div id="viewer">
        <p id="filename">{{filename}}</p>
        <textarea class="preview">山路を登りながら、こう考えた。智に働けば角が立つ。</textarea>
        <input type="text" id="path_field">
        <button id="path_loadbutton">LOAD</button>
        <p id="message" v-if="isinput">{{ message }}</p>
        <p id="path">{{ args }}</p>
    </div>

</template>

<script>
import {convertFileSrc} from '@tauri-apps/api/tauri';

export default {
    name: "Main.vue",
    data() {
        return {
            filename: 'ssss',
            isinput: true,
            args: [],
            fontt: null,
            message: 'なんか入力して'


        }
    },
    mounted: function () {
        console.log('aa');
        this.getArgs();


    },
    methods: {

        getArgs: function () {
            this.$data.isinput = false;
            var invoke = window.__TAURI__.invoke

            console.log('!')
            var vm = this.$data
            invoke('get_args', {})
                .then(function (rustMsg) {

                    /*this.$data.args = rustMsg;
                    console.log(this.$data.args)*/
                    /*
                                        document.getElementById("path").value = rustMsg;
                    */
                    /*
                                        document.getElementById("path").value = "rustMsg";
                                        rustMs
                    */
                    vm.args = rustMsg.split(",")
                    if (vm.args[1] != null) {

                    } else {
                        vm.args[1] = "C:\\Users\\ym174\\OneDrive\\デスクトップ\\TBGoStdR-C6.otf"
                    }
                    alert("url(" + vm.args[1] + ")")
                    vm.fontt = new FontFace("LoadedFont", "url(" + convertFileSrc(vm.args[1]) + ")")
                    vm.fontt.load().then(function (loadedFace) {
                        /// フォント読み込み成功
                        /// body要素全体にそれを適用する
                        document.fonts.add(loadedFace);
                        var splited_path = vm.args[1].split('\\')
                        alert(splited_path[splited_path.length - 1])
                        vm.filename = splited_path[splited_path.length - 1]
                        /*
                                                    document.body.style.fontFamily = '"Cosmos Logic"';
                        */
                    }).catch(function (e) {
                        /// フォント読み込み失敗
                        alert('Failed to load font');
                        vm.message = '読み込めんかった';
                    });


                })// 戻り値を表示
        },
        load_font: function () {

        }
    }

}
</script>

<style scoped>

@import "stylesheets/default.css";

.preview {
    border: none;
    resize: none;
    font-size: 2em;
    font-family: 'LoadedFont'
}

.preview :focus {
    outline: none;
    background: #000;
}

#filename{
    
}
</style>