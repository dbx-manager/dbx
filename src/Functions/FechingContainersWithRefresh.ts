import {  ref, watch } from "vue";
import { useSettings } from "../composables/useSettings";
import { ListContainer } from "../Types/ListContainer";
import { list_containers } from "./ListingContaienrs";

export const containerList = ref<ListContainer[] | null>(null);
export const { settings } = useSettings();
const isLoading = ref(false);
const updateError = ref<string | null>(null);
let updateInterval: number | null = null;


// Function to fetch container list with error handling
export const fetchContainerList = async () => {
    if (!settings.autoUpdateEnabled) return;

    try {
        isLoading.value = true;
        updateError.value = null;
        const newList = await list_containers();
        containerList.value = newList;
    } catch (error) {
        console.error("Failed to fetch container list:", error);
        updateError.value =
            error instanceof Error
                ? error.message
                : "Failed to fetch containers";
    } finally {
        isLoading.value = false;
    }
};

// Start the update interval
export const startUpdateInterval = () => {
    if (updateInterval) {
        clearInterval(updateInterval);
    }

    if (settings.autoUpdateEnabled && settings.updateInterval > 0) {
        updateInterval = setInterval(
            fetchContainerList,
            settings.updateInterval,
        );
    }
};

// Stop the update interval
 export const stopUpdateInterval = () => {
    if (updateInterval) {
        clearInterval(updateInterval);
        updateInterval = null;
    }
};

// Watch for settings changes and update interval accordingly
watch(
    () => settings.updateInterval,
    () => {
        startUpdateInterval();
    },
);

watch(
    () => settings.autoUpdateEnabled,
    (enabled) => {
        if (enabled) {
            startUpdateInterval();
        } else {
            stopUpdateInterval();
        }
    },
);
