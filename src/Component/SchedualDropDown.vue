<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { formatCronToTime, formatCronToDate } from "../utils/cronUtils";
import { AppSettings } from "../composables/useSettings";

const SelectedItem = ref<string>("Daily");
const pickerContainer = ref<any>(null);
const TimeVisibility = ref<boolean>(true);
const DateVisibility = ref<boolean>(true);
const items = ["Daily", "Weekly", "Monthly"];
const handleClickOutside = (event:PointerEvent) => {
    if (
        pickerContainer.value &&
        !pickerContainer.value.contains(event.target)
    ) {
        TimeVisibility.value = true;
        DateVisibility.value = true;
    }
};
onMounted(() => {
    document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
    document.removeEventListener("click", handleClickOutside);
});
// Date Constraints (Days 1-28 only)
// We set min/max to a specific month/year to prevent navigation
const onDateSelect = (selectedDate:any) => {
    const day = selectedDate.getDate();
};


const currentYear = new Date().getFullYear();
const currentMonth = new Date().getMonth(); // 0-indexed

// Set range to 1st to 28th of the current month/year
const minDate = new Date(currentYear, currentMonth, 1)
    .toISOString()
    .substring(0, 10);
const maxDate = new Date(currentYear, currentMonth, 28)
    .toISOString()
    .substring(0, 10);
// Extra safety: Disable any date not between 1 and 28 (in case logic shifts)
const allowedMonthes = (date:any) => {
    const day = new Date(date).getDate();
    return day >= 1 && day <= 28;
};

interface Props {
    settings: AppSettings;
}
const props = defineProps<Props>();
</script>
<template>
    <div class="flex flex-row items-center gap-2" ref="pickerContainer">
        <label class="text-sm">Schedual:</label>
        <v-menu >
            <template v-slot:activator="{ props }">
                <v-btn
                    flat
                    color="#22222698"
                    class="w-fit text-sm! px-2! h-fit! py-2"
                    v-bind="props"
                >
                    {{ SelectedItem }}
                    <v-icon color="white" size="18px" class="pt-1 pl-2"
                        >mdi-chevron-down</v-icon
                    >
                </v-btn>
            </template>

            <v-list v-model="SelectedItem" density="compact">
                <v-list-item
                    v-for="item in items"
                    :key="item"
                    :value="item"
                    v-on:click="SelectedItem = item"
                >
                    <v-list-item-title>{{ item }}</v-list-item-title>
                </v-list-item>
            </v-list>
        </v-menu>
        <label class="text-sm">At:</label>
        <div class="flex justify-center">
            <v-btn
            flat
            
             color="#22222698"
                @Click="
                    {
                        TimeVisibility = !TimeVisibility;
                        DateVisibility = true;
                    }
                "
            :text="formatCronToTime(settings.cronSchedule)"    
            
                />
            
            <v-time-picker
                :hidden="TimeVisibility"
                rounded="lg"
                class="absolute z-50 mt-10"
            />
        </div>
        <div class="flex justify-center">
            <v-btn
            flat
             color="#22222698"
            :text="formatCronToDate(settings.cronSchedule)"    
            :hidden="SelectedItem.match('Monthly')==null"
                @Click="
                    {
                        DateVisibility = !DateVisibility;
                        TimeVisibility = true;
                    }
                "
            >
            </v-btn>
            <v-date-picker
                :hidden="DateVisibility"
                v-model="Date"
                :min="minDate"
                :max="maxDate"
                controlVariant="modal"
                hide-header
                hide-title
                no-month-picker
                no-year-picker
                hide-weekdays
                :allowed-dates="allowedMonthes"
                rounded="lg"
                @update:model-value="onDateSelect"
                class="absolute z-50 mt-10 shadow-black! shadow-lg/20!"
            />
        </div>
    </div>
</template>
