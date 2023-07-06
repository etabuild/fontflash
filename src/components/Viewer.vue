<script setup>
import {ref, reactive, computed, toRefs, watch} from "vue";
import {watchEffect} from "vue";

const props = defineProps({
    filepath: String
})

let filename = computed(() => {
    const splitedPath = props.filepath.split("\\")
    return splitedPath[splitedPath.length - 1]
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

        <div id="font-info">
            <p id="family-name">{{ filename }}</p>
            <p id="control-font-weight">bold (400)</p>


            <p id="filename">{{ props.filepath }}</p>
        </div>

        <div class="font-preview">
            <div class="font_size_control">
                <p class="p_size">{{ fontsize }}</p>
                <input class="range" id="range_fontsize_1"
                       v-model.number="config.previewFontSizeRate" type="range"/>
                <input class="range" id="range-font-weight" v-model.number="config.fontWeight" type="range">
            </div>
            <button class="b-install-font" @click="installFont()">インストール</button>

            <div id="preview_area">
                <!--            <textarea id="preview0" :style="{fontSize: fontsize}"
                                      class="preview">あのイーハトーヴォのす/\きとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。</textarea>-->
                <p class="preview-text" style="{font-variation-settings: 'wght' 300;font-size:2em}">
                    Loremしかし私から何にも聞かないＫは、いつもよりなお黙っていたところで、素性の知れない人は厭だと答えるのです。</p>
                <p class="preview-text" :style="{fontVariationSettings:['wght',1000],fontSize:fontsize}">
                    123456 Lorem ipsum dolor sit amet, consectetur adipiscing elit</p>
            </div>
        </div>
        <div id="footer_area">
        </div>
    </div>
</template>

<style scoped>
#range-font-weight{
    display: inline-block;
}
#range_fontsize_1{
    display: inline-block;

}
.b-install-font{
    display: inline-block;
}
.font-preview{
    grid-row: 2/3;
    padding: 0 12px;
    overflow-y: scroll;
    height: 100%;
    word-break: break-all;
    overflow-x:auto;
}
#font-info {
    grid-row: 1/2;

    background: #e2f0ff;
    padding: 5px;
    border-radius: 5px;
}

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
    color: #000000;
    font-size: 1.4em;
    word-break: break-all;
    display: inline-block;
    margin-top: 0px;
    padding-top: 2px;
    user-select: text;

}

#control-font-weight {
    background: #20ffaa;
    color: #000;
    padding: 1px 7px;
    width: auto;
    display: inline-block;
    border-radius: 6px;
    margin: 0px 0px 0px 5px;
    font-size: 0.9em;
}



/*
#filename {
    display: inline-block;
    padding: 0px 4px;

}*/

#filename {
    word-break: break-all;
    user-select: text;

}

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