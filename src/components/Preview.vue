<script setup>
import {computed, reactive} from "vue";

let fontsize = computed(() => Math.round(config.previewFontSizeRate * 1.4) + 10 + 'pt')
let fontWeight = computed(()=> config.fontWeightRate * 10)
let config = reactive({
    previewFontSizeRate: 11,
    fontWeightRate: 10
})

</script>

<template>
    <div id="preview_area">
        <div class="font_size_control">
            <p class="p_size">{{ fontsize }}</p>
            <input class="range" id="range_fontsize_1"
                   v-model.number="config.previewFontSizeRate" type="range"/>
            <input class="range" id="range-font-weight" v-model.number="config.fontWeightRate" type="range">
        </div>

        <!--            <textarea id="preview0" :style="{fontSize: fontsize}"
                              class="preview">あのイーハトーヴォのす/\きとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。</textarea>-->
        <textarea class="preview-text" style="{font-variation-settings: 'wght' 300;font-size:2em}">Loremしかし私から何にも聞かないＫは、いつもよりなお黙っていたところで、素性の知れない人は厭だと答えるのです。
                </textarea>
        <textarea class="preview-text" readonly
                  :style="{fontVariationSettings:`'wght' `+fontWeight,fontSize:fontsize}">123456 Lorem ipsum dolor sit amet, consectetur adipiscing elit
                </textarea>
    </div>
</template>

<style scoped>
.preview-text {
    display:block;
    background: none;
    overflow: hidden;
    border:none;
    height: auto;
    width: 100%;
    resize: none;
    font-family: 'LoadedFont';
    font-size: 1.4em;
    /*
    font-variation-settings: 'wght' 1000;
    */
}
</style>