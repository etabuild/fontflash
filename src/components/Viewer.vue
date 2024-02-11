<script setup>
import {ref, reactive, computed, toRefs, watch} from "vue";
import {watchEffect} from "vue";
import Preview from "./Preview.vue";
import Metadata from "./Metadata.vue";

const props = defineProps({
    filepath: String,
    nameRecords: [],
    familyName: String,
    psName:String,
    weight: String
})

let filename = computed(() => {
    const splitedPath = props.filepath.split("\\")
    return splitedPath[splitedPath.length - 1]
})

let previewFontWeight = computed(() =>
    Math.round(config.fontWeight * 10)
)

/*const {fileName} = toRefs(props)*/
const currentTab = ref('preview')
const switchTab = (target) => {
    console.log(target)
    if(target === 'preview' || 'metadata') {

    }
    currentTab.value = target

    console.log(currentTab.value)
}
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

            <button class="b-install-font" @click="installFont()">インストール</button>
            <div class="tablist">
                <div id="tab_preview"  v-bind:class="{'tab':true,'active':currentTab==='preview'}" @click="switchTab('preview')">
                    <p>プレビュー</p>
                </div>
                <div id="tab_metadata"   v-bind:class="{'tab':true,'active':currentTab==='metadata'}" @click="switchTab('metadata')">
                    <p>メタデータ</p>
                </div>
            </div>

            <Preview v-if="currentTab==='preview'"></Preview>
            <Metadata v-if="currentTab==='metadata'"></Metadata>

        </div>
        <div id="footer_area">
        </div>
    </div>
</template>

<style scoped>

.tab {
    display: inline-block;
    margin: 7px;
    color: #62acff;

}

.tab:hover {
    color: #1675da
}

.tab.active:hover:after {
    background: #1675da;
    height: 3px;
    border-radius: 99px;
    content: '';
    display: block;
}

.tab.active:after {
    background: #1886fa;
    height: 3px;
    border-radius: 99px;
    content: '';
    display: block;
}
.tab.active {
    color: #1886fa;

}


#range-font-weight {
    display: inline-block;
}

#range_fontsize_1 {
    display: inline-block;

}

.b-install-font {
    display: inline-block;
}

.font-preview {
    grid-row: 2/3;
    padding: 0 12px;
    overflow-y: scroll;
    height: 100%;
    word-break: break-all;
    overflow-x: auto;
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


</style>