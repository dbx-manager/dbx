<script lang="ts" setup>
interface Props {
    tooltip: string;
    checkboxLable: string;
    id?:string;
    onchangeFunction?:(id:string,state:boolean)=>void
}
function s(e:Event){
    const target = e.target as HTMLInputElement;
    console.log(target.checked)
    if(props.onchangeFunction&&props.id)
        props.onchangeFunction(props.id,target.checked)
    
}
const props = defineProps<Props>();
</script>
<template>
    <div>
        <v-tooltip
            activator="parent"
            location="top"
            offset="0"
            interactive
            open-delay="1000"
            maxWidth="200"
            :text="tooltip"
        />
        <v-checkbox-btn :label="checkboxLable" @change="s"/>
    </div>
</template>
