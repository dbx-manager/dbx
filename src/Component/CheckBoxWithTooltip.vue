<script lang="ts" setup>
interface Props {
    tooltip: string;
    checkboxLable: string;
}
defineProps<Props>();
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
        <v-checkbox-btn :label="checkboxLable" />
    </div>
</template>
