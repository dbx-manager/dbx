<script lang="ts" setup>
import { ref } from 'vue';
interface Props {
    dialog: boolean;
}
const dialog=ref<boolean>(false)
defineProps<Props>()
</script>
<template>
    <v-btn variant="flat" size="large" density="default" color="#3584e4ff" class="rounded-md"
                @Click="dialog = true"><span class="text-lg font-bold">New Container
                </span></v-btn>
    <v-dialog
    
      v-model="dialog"
      width="auto"
    >
      <v-card
        max-width="400"
        prepend-icon="mdi-update"
        text="Your application will relaunch automatically after the update is complete."
        title="Update in progress"
      >
        <template v-slot:actions>
          <v-btn
            class="ms-auto"
            text="Ok"
            @click="dialog = false"
          ></v-btn>
        </template>
      </v-card>
    </v-dialog>
</template>