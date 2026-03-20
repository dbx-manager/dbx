<script setup lang="ts">
interface Props {
    status: string;
    contianerId:string;
}
defineProps<Props>();
</script>

<template>
    <!--TODO: apply your actions (start stop pause unpause.... ) on those buttons invoking the right functions  -->
    <v-btn text="Start"  color="#188251ff" rounded="lg" flat class=""></v-btn>
    <v-btn  text="Stop"  color="#9a2a2aff" rounded="lg" flat class=""></v-btn>
    <span  class="flex flex-col" :class="{ 'text-green-600':status.match('running'),'text-red':!status.match('running')}" v-if="status" > {{status}}</span>
    <v-btn  text="Pause" color="#6c1882ff" rounded="lg" flat class=""></v-btn>
    <v-btn  text="Unpause" color="#2a439aff" rounded="lg" flat class=""/>
    <span class="flex flex-col ">Auto Start</span>
    <v-btn text="View Arguments" class="col-span-2"  rounded="lg"> </v-btn>
    <v-btn text="Delete" color="#bd1919ff" rounded="lg" class=""> </v-btn>
</template>
