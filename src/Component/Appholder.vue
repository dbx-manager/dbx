<script setup lang="ts">
interface Props {
    appInfo: String;
    iconUrl?: string;
    selectionBox?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
    selectionBox: false,
    iconUrl: "",
});
</script>

<template>
    <li class="flex flex-row items-center gap-2">
        <input
            type="checkbox"
            v-if="selectionBox"
            class="w-5 h-5 z-10 bg-transparent! rounded-sm! forced-colors:appearance-auto border border-blue-700! focus:ring-1 focus:ring-offset-0! focus:outline-none!"
        />
        <v-icon>mdi-application-cog-outline</v-icon>
        <div class="flex flex-col truncate text-ellipsis!">
            <v-tooltip> ss </v-tooltip>

            <span class="w-fit z-0"
                >{{ appInfo ? appInfo.split(":")[0] : "Error Happened" }}
                <v-tooltip
                    activator="parent"
                    location="top"
                    offset="0"
                    interactive
                    open-delay="1000"
                    close-delay="1000"
                    class=""
                    >{{
                        appInfo ? appInfo.split(":")[0] : "Error Happened"
                    }}</v-tooltip
                ></span
            >
            <span class="text-xs text-gray-500 z-0"
                >{{ appInfo ? appInfo.split(":")[1] : "Error Happened" }}
                <v-tooltip
                    activator="parent"
                    location="top"
                    offset="0"
                    interactive
                    open-delay="1000"
                    close-delay="1000"
                    class=""
                    >{{
                        appInfo ? appInfo.split(":")[1] : "Error Happened"
                    }}</v-tooltip
                >
            </span>
        </div>
    </li>
</template>
