<script setup>
import { onMounted, onUnmounted, ref } from "vue";

const SelectedItem = ref("Daily");
const pickerContainer = ref(null);
const TimeVisibility = ref(true);
const DateVisibility = ref(true);
const items = ["Daily", "Weekly", "Monthly", "Yearly"];
const handleClickOutside = (event) => {
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
</script>
<script>
const onDateSelect = (selectedDate) => {
    const day = selectedDate.getDate();
};
// Date Constraints (Days 1-28 only)
// We set min/max to a specific month/year to prevent navigation
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
const allowedMonthes = (date) => {
    const day = new Date(date).getDate();
    return day >= 1 && day <= 28;
};
</script>
<template>
    <div class="flex flex-row items-center gap-2" ref="pickerContainer">
        <label class="text-sm">Schedual:</label>
        <v-menu :location="location">
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
                @Click="
                    {
                        TimeVisibility = !TimeVisibility;
                        DateVisibility = true;
                    }
                "
            >
            </v-btn>
            <v-time-picker
                :hidden="TimeVisibility"
                class="absolute z-50 mt-10"
            />
        </div>
        <div class="flex justify-center">
            <v-btn
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
                v-model="date"
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
