<script setup>
import {ref, reactive, computed} from "vue";
let font = reactive({
    familyName: "familynameHere",
    fontWeight: "000"
})
const fontsize = computed(()=>Math.round(config.previewFontSizeRate * 1.4) + 10 + 'pt')
let config = reactive({
    previewFontSizeRate:11
})
/*
let fontsize = 500
*/
let filename = "name"
</script>

<template>
    <div id="viewer">

        <div class="font_description">
            <p id="family-name">{{ font.familyName }}</p>
            <div id="font-info">
                <p id="control-font-weight" v-if="font.hasWeight">{{ font.fontWeight }}</p>
                <p id="filename">{{ filename }}</p>
            </div>


            <div class="font_size_control">
                <p class="p_size">{{ fontsize }}</p>
                <input class="range" id="range_fontsize_1"
                       v-model.number="config.previewFontSizeRate" type="range"/>
            </div>
        </div>
        <div id="preview_area">
            <textarea id="preview0" :style="{fontSize: fontsize}"
                      class="preview">あのイーハトーヴォのすきとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。</textarea>
        </div>
        <div id="footer_area">
        </div>
    </div>
</template>

<style scoped>

.font_description {
    display: grid;
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
    border: none;
    resize: none;
    /*
    font-size: 400pt;
    */
    font-family: 'LoadedFont';
    margin: 5px;
    background: none;
    width: calc(100% - 10px);
    line-height: 1.2;
    overflow: scroll;
    background: #ffffff;
    border-radius: 10px;
    padding: 10px;
    height: 90%;

}/*
#filename {
    display: inline-block;
    padding: 0px 4px;

}*/
.preview :focus {
    outline: none;
    background: #000;
}
</style>