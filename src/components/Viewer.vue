<script setup>
import {ref, reactive, computed, toRefs, watch} from "vue";
import {watchEffect} from "vue";

const props = defineProps({
    fileName: String
})

let previewFontWeight = computed(() =>
    Math.round(config.fontWeight * 10)
)

/*const {fileName} = toRefs(props)*/
let fontsize = computed(() => Math.round(config.previewFontSizeRate * 1.4) + 10 + 'pt')
let config = reactive({
    previewFontSizeRate: 11,
    fontWeight: 10
})


function installFont() {

}

/*
let fontsize = 500
*/
</script>

<template>
    <div id="viewer">

        <div class="font_description">
            <p id="family-name">{{ props.fileName }}</p>
            <div id="font-info">
                <p id="control-font-weight">{{ }}</p>
                <p id="filename">{{ props.fileName }}</p>
            </div>


            <div class="font_size_control">
                <p class="p_size">{{ fontsize }}</p>
                <input class="range" id="range_fontsize_1"
                       v-model.number="config.previewFontSizeRate" type="range"/>
                <input class="range" id="range-font-weight" v-model.number="config.fontWeight" type="range">
            </div>
            <button class="b-install-font" @click="installFont()">インストール</button>
        </div>
        <div id="preview_area">
            <!--            <textarea id="preview0" :style="{fontSize: fontsize}"
                                  class="preview">あのイーハトーヴォのす/\きとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。</textarea>-->
            <p class="preview-text" style="{font-variation-settings: 'wght' 300;font-size:2em}">
                Loremしかし私から何にも聞かないＫは、いつもよりなお黙っていたところで、素性の知れない人は厭だと答えるのです。</p>
            <p class="preview-text" :style="{fontVariationSettings:['wght',1000],fontSize:fontsize}">
                123456 Lorem ipsum dolor sit amet, consectetur adipiscing elit</p>
        </div>
        <div id="footer_area">
        </div>
    </div>
</template>

<style scoped>

.font_description {
    /*
    display: grid;
    */
    grid-template-columns: 1fr auto;
    grid-template-rows: 2fr 1fr;
}

.font_size_control {

    margin: 5px;
    padding: 5px;
}

.p_size {
    display: inline-block;
    font-weight: bold;

}

.range {
    margin: 5px 0px 5px 5px
}

#family-name {
    font-weight: 600;
    color: #000;
    font-size: 1.8em;
    margin-left: 2px;
}

#control-font-weight {
    background: #20ffaa;
    color: #000;
    padding: 2px 7px;
    width: auto;
    display: inline-block;
    border-radius: 6px;
}


.preview {
    resize: none;
    /*
    font-size: 400pt;
    */
    font-family: 'LoadedFont';
    margin: 5px;
    background: none;
    width: calc(100% - 10px);
    line-height: 1.2;
    overflow: hidden;
    background: #ffffff;
    border-radius: 10px;
    padding: 10px;
    height: 90%;
    border: #8c8c8c solid 1px;

}

/*
#filename {
    display: inline-block;
    padding: 0px 4px;

}*/
.preview :focus {
    outline: none;
    background: #000;
}

.preview-text {

    font-family: 'LoadedFont';
    font-size: 1.4em;
    /*
    font-variation-settings: 'wght' 1000;
    */
}
</style>