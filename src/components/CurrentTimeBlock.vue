<script setup lang="ts">
import { ref, onMounted, onUnmounted, StyleValue } from "vue";

const props = defineProps<{
  class?: StyleValue;
}>();

const date = ref(new Date());

let intervalId: number | undefined;

onMounted(() => {
  intervalId = window.setInterval(() => {
    date.value = new Date();
  }, 5 * 1000);
});

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId);
});
</script>

<template>
  <div
    class="flex flex-col gap-2 justify-center p-6 rounded-lg shadow-lg bg-radial-[at_90%_90%] from-blue-900 from-20% to-blue-950 to-80% text-white"
    :class="props.class"
  >
    <p class="text-5xl font-bold text-center">
      {{ $d(date, { hour: "2-digit", minute: "2-digit" }) }}
    </p>
    <p class="text-xl text-center">{{ $d(date) }}</p>
  </div>
</template>
