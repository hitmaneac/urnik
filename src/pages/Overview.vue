<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface PunchLogDto {
  id: number;
  card_number: string;
  card_name?: string | null;
  user_fullname?: string | null;
  status: string;
  timestamp: string;
}

const punchLog = ref<PunchLogDto[]>([]);

import { computed } from "vue";
const hoursWorkedByCard = computed(() => {
  const grouped: Record<string, PunchLogDto[]> = {};
  for (const punch of punchLog.value) {
    if (!grouped[punch.card_number]) grouped[punch.card_number] = [];
    grouped[punch.card_number].push(punch);
  }
  const result: Record<string, number> = {};
  for (const [card, punches] of Object.entries(grouped)) {
    punches.sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime());
    let totalMs = 0;
    let inTime: Date | null = null;
    for (const punch of punches) {
      if (punch.status === "In") {
        inTime = new Date(punch.timestamp);
      } else if (inTime) {
        const outTime = new Date(punch.timestamp);
        totalMs += outTime.getTime() - inTime.getTime();
        inTime = null;
      }
    }
    result[card] = totalMs / 1000 / 3600; // hours
  }
  return result;
});
const errorMsg = ref<string | null>(null);
const isLoading = ref(false);


async function resetPunchLog() {
  isLoading.value = true;
  errorMsg.value = null;
  try {
    await invoke("clear_punch_logs");
    await fetchPunchLog();
  } catch (e: any) {
    errorMsg.value = e?.toString() || "Unknown error";
  } finally {
    isLoading.value = false;
  }
}

async function fetchPunchLog() {
  isLoading.value = true;
  errorMsg.value = null;
  try {
    punchLog.value = await invoke("get_punch_log");
  } catch (e: any) {
    errorMsg.value = e?.toString() || "Unknown error";
  } finally {
    isLoading.value = false;
  }
}

onMounted(fetchPunchLog);
</script>

<template>
  <div>
    <h2 class="text-xl font-bold mb-4">{{$t('Punch Log')}}</h2>
    <button class="mb-2 px-3 py-1 bg-blue-700 text-white rounded mr-2" @click="fetchPunchLog" :disabled="isLoading">
      {{$t('Refresh')}}
    </button>
    <button class="mb-2 px-3 py-1 bg-red-700 text-white rounded" @click="resetPunchLog" :disabled="isLoading">
      {{$t('Reset Logs')}}
    </button>
    <div v-if="isLoading" class="text-blue-400">{{$t('Loading...')}}</div>
    <div v-if="errorMsg" class="text-red-400">{{ errorMsg }}</div>

    <h3 class="text-lg font-semibold mt-6 mb-2">{{$t('Cards Summary')}}</h3>
    <table v-if="Object.keys(hoursWorkedByCard).length" class="w-full border mb-6">
      <thead>
        <tr class="bg-gray-700 text-white">
          <th class="p-2 text-left">{{$t('Name')}}</th>
          <th class="p-2 text-left">{{$t('Card Number')}}</th>
          <th class="p-2 text-left">{{$t('Total Hours Worked')}}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="card in Object.keys(hoursWorkedByCard)" :key="card">
          <td class="p-2">
            {{
              (punchLog.find(e => e.card_number === card)?.card_name) + ' ' + (punchLog.find(e => e.card_number === card)?.user_fullname)
            }}
          </td>
          <td class="p-2 text-left">{{ card }}</td>
          <td class="p-2 text-left">
            <template v-if="typeof hoursWorkedByCard[card] === 'number'">
              {{ Math.floor(hoursWorkedByCard[card]) }}h {{ Math.round((hoursWorkedByCard[card] % 1) * 60) }}m
            </template>
            <template v-else>
              0h 0m
            </template>
          </td>
        </tr>
      </tbody>
    </table>

    <table v-if="punchLog.length" class="w-full border mt-2">
      <thead>
        <tr class="bg-gray-800 text-white">
          <th class="p-2 text-left">{{$t('Name')}}</th>
          <th class="p-2 text-left">{{$t('Card Number')}}</th>
          <th class="p-2 text-left">{{$t('Status')}}</th>
          <th class="p-2 text-left">{{$t('Date & Time')}}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="event in punchLog" :key="event.id">
          <td class="p-2 text-left">{{ event.user_fullname || event.card_name || event.card_number }}</td>
          <td class="p-2 text-left">{{ event.card_number }}</td>
          <td class="p-2 text-left font-bold">{{ $t(event.status) }}</td>
          <td class="p-2 text-left">{{ event.timestamp }}</td>
        </tr>
      </tbody>
    </table>
    <div v-else-if="!isLoading && !errorMsg" class="text-gray-400">{{$t('No data.')}}</div>
  </div>
</template>
