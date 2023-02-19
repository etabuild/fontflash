import {createApp} from "vue";
import "./style.css";
import App from "./App.vue";
/*
import {getMatches} from '@tauri-apps/api/cli';/!*for(var i = 0;i < process.argv.length; i++){
    console.log("argv[" + i + "] = " + process.argv[i]);
}*!/
*/
createApp(App).mount("#app");
/*const invoke = window.__TAURI__.invoke;
var args;
const getArgs = () => {
    console.log('!')
    invoke('get_args', {})
        .then(function (rustMsg) {

            args = rustMsg;
            alert(args)
        })// 戻り値を表示
};*/
