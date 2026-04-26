<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from "vue";
import { formatCronToTime, formatCronToDate, CronExp } from "../utils/cronUtils";

interface Props {
    cron_schedual: string;
    cron_sync_function: (c: CronExp) => void;
}

const props = defineProps<Props>();

const SelectedItem = ref<string>("Daily");
const FinalCron = ref<CronExp | null>(null);
const pickerContainer = ref<any>(null);
const TimeVisibility = ref<boolean>(true);
const DateVisibility = ref<boolean>(true);

const items = ["Daily", "Monthly"];


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

const handleClickOutside = (event:PointerEvent) => {
    if (
        pickerContainer.value &&
        !pickerContainer.value.contains(event.target)
    ) {
        TimeVisibility.value = true;
        DateVisibility.value = true;
    }
};

const initializeCron = (cronString: string) => {
  // Cleanup existing instance if exists
  if (FinalCron.value) {
    FinalCron.value=null;
  }

  // Create fresh new CronExp with updated value
  FinalCron.value = new CronExp(props.cron_sync_function, cronString);
  
  // Auto-detect and set correct UI state
  const dayField = FinalCron.value.getFields().day;
  SelectedItem.value = dayField === '*' ? 'Daily' : 'Monthly';
};

onMounted(() => {
    document.addEventListener("click", handleClickOutside);
    
    // Initialize on mount
    if (props.cron_schedual) {
      initializeCron(props.cron_schedual);
    }
});

// ✅ Reactive watcher for prop changes
watch(() => props.cron_schedual, (newCronValue) => {
  if (newCronValue && FinalCron.value?.toString() !== newCronValue) {
    initializeCron(newCronValue);
  }
}, { immediate: false });

onUnmounted(() => {
    document.removeEventListener("click", handleClickOutside);
    
    // Proper cleanup on unmount
    if (FinalCron.value) {
      FinalCron.value=null;
      FinalCron.value = null;
    }
});
// Date Constraints (Days 1-28 only)
// We set min/max to a specific month/year to prevent navigation
const onDateSelect = (selectedDate:any) => {
    if(!selectedDate){return}
    const s=selectedDate.getDate()
    FinalCron.value?.setField('day',s)
    // console.log(selectedDate.getDay())
    // console.log(FinalCron.value?.getFields())

};
function onTimeSelect(newTime:string|null) {
    if(!newTime){return}
    const s=newTime.split(':')
    FinalCron.value?.setField('hour',s[0])
    FinalCron.value?.setField('min',s[1])
    // console.log(FinalCron.value?.getFields())

} 
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
                    v-on:click="{SelectedItem = item;FinalCron?.setField('day','*')}"
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
            :text="formatCronToTime(FinalCron?.toString()??' ')"    
            
                />
            
            <v-time-picker
                :hidden="TimeVisibility"
                rounded="lg"
                @update:model-value="onTimeSelect"
                class="absolute z-50 mt-10"
            />
        </div>
        <div class="flex justify-center">
            <v-btn
            flat
             color="#22222698"
            :text="formatCronToDate(FinalCron?.toString()??'')"    
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
