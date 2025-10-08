<script setup lang="ts">
import CurrentTimeBlock from "../components/CurrentTimeBlock.vue";
import Button from "../components/Button.vue";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";
import Lunch from "../assets/lunch.svg";
import Vacation from "../assets/vacation.svg";
import Business from "../assets/business-time.svg";
import Break from "../assets/break.svg";
import Personal from "../assets/time.svg";
import Sick from "../assets/sick.svg";

const { t } = useI18n();

const buttons = [
  { icon: Lunch, label: "leave_type.lunch" },
  { icon: Vacation, label: "leave_type.vacation" },
  { icon: Business, label: "leave_type.business" },
  { icon: Break, label: "leave_type.break" },
  { icon: Personal, label: "leave_type.personal" },
  { icon: Sick, label: "leave_type.sick" },
];


const card_uid = ref<string | null>(null);
const user = ref<any | null>(null);
const leaveResult = ref<string | null>(null);
const errorMsg = ref<string | null>(null);
const isLoading = ref(false);
const punchLog = ref<any[]>([]);

const pendingLeaveType = ref<string | null>(null);
let leaveTimeout: ReturnType<typeof setTimeout> | null = null;

const testUids = [
  [0xAA, 0xBB, 0xCC, 0x01],
  [0xAA, 0xBB, 0xCC, 0x02],
  [0xAA, 0xBB, 0xCC, 0x03],
];
import { ref, computed, onMounted } from "vue";
async function fetchPunchLog() {
  try {
    punchLog.value = await invoke("get_punch_log");
  } catch (e) {
    // error
  }
}

onMounted(fetchPunchLog);
if (typeof window !== 'undefined') {
  window.addEventListener('focus', fetchPunchLog);
}

const selectedTestUidIdx = computed(() => {
  if (!selectedTestUid.value) return null;
  return testUids.findIndex(u => Array.isArray(selectedTestUid.value) && selectedTestUid.value.length === u.length && selectedTestUid.value.every((v, i) => v === u[i]));
});
const selectedTestUid = ref<number[] | null>(null);

const selectedTestUidHex = computed(() => {
  if (Array.isArray(selectedTestUid.value)) {
    return selectedTestUid.value.map((b: number) => b.toString(16).padStart(2, '0')).join(':');
  }
  return '';
});

async function scanCard() {
  card_uid.value = null;
  user.value = null;
  leaveResult.value = null;
  errorMsg.value = null;
  isLoading.value = true;
  try {
    const isLeave = !!pendingLeaveType.value;
    const leaveType = pendingLeaveType.value || null;
    const args: Record<string, unknown> = { timeout: 10, isLeave, leaveType };
    if (selectedTestUid.value) {
      args.uid = selectedTestUid.value;
    }
    const uid = await invoke("read_card_uid", args) as number[] | null;
    if (!uid || uid.length === 0) {
      isLoading.value = false;
      return;
    }
    selectedTestUid.value = null;
    const uidHex = uid.map((b) => b.toString(16).padStart(2, "0")).join(":");
    card_uid.value = uidHex;
    const userData = await invoke("find_or_create_user", { cardUid: uidHex, userFullname: null }) as { card_name?: string, user_fullname?: string, card_number?: string };
    user.value = userData;
    const name = [userData.card_name, userData.user_fullname].filter(Boolean).join(' ');
    window.dispatchEvent(new CustomEvent('urniknet-user', { detail: name || userData.card_number }));
    
  punchLog.value = await invoke("get_punch_log");
      const lastPunch = punchLog.value.find((p: any) => p.card_number === uidHex);
      if (pendingLeaveType.value) {
        // If leave is pending and card is currently punched in, register leave (punch out)
        if (lastPunch && lastPunch.status === "In") {
          const result: string = await invoke("register_leave", { cardUid: uidHex, leaveType: pendingLeaveType.value });
          leaveResult.value = result;
        } else if (lastPunch) {
          leaveResult.value = t("You are not punched in. Please punch in first.");
        } else {
          leaveResult.value = t("No punch record found. Please punch in first.");
        }
        pendingLeaveType.value = null;
        if (leaveTimeout) clearTimeout(leaveTimeout);
      } else {
        if (!lastPunch || lastPunch.status !== "In") {
          leaveResult.value = t("Punched Out") + (lastPunch && lastPunch.status ? ` (${lastPunch.status})` : '');
        } else {
          leaveResult.value = t("Punched In");
        }
      }
  } catch (e: any) {
    errorMsg.value = e?.toString() || t("Unknown error");
  } finally {
    isLoading.value = false;
  }
}

function selectTestCard(selectedUid: number[] | null) {
  pendingLeaveType.value = null;
  selectedTestUid.value = selectedUid;
  if (selectedUid) {
    leaveResult.value = `UID ${selectedUid.map(b => b.toString(16).padStart(2, '0')).join(':')}`;
  } else {
    leaveResult.value = null;
  }
}

async function handleButtonClick(leaveTypeLabel: string) {
  card_uid.value = null;
  user.value = null;
  leaveResult.value = null;
  errorMsg.value = null;
  isLoading.value = false;
  pendingLeaveType.value = leaveTypeLabel;
  leaveResult.value = leaveTypeLabel;
  if (leaveTimeout) clearTimeout(leaveTimeout);
  leaveTimeout = setTimeout(() => {
    pendingLeaveType.value = null;
    leaveResult.value = null;
    card_uid.value = null;
    user.value = null;
    selectedTestUid.value = null;
    window.dispatchEvent(new CustomEvent('urniknet-user', { detail: t('message.bye') }));
  }, 10000);
}

const getCardButtonClass = (uid: number[], idx: number) => {
  const uidHex = uid.map((b) => b.toString(16).padStart(2, "0")).join(":");
  const punches = punchLog.value.filter((p: any) => p.card_number === uidHex)
    .sort((a: any, b: any) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime());
  const lastPunch = punches.length > 0 ? punches[0] : null;
  if (lastPunch && lastPunch.status === "In") {
    return 'bg-green-500 text-white';
  } else if (lastPunch && (lastPunch.status === "Out" || lastPunch.status.startsWith('leave_type.'))) {
    return 'bg-red-500 text-white';
  } else if (!lastPunch && selectedTestUidIdx.value === idx) {
    return 'bg-blue-500 text-white';
  } else {
    return '';
  }
}

</script>

<template>
  <div class="grid grid-cols-6 grid-rows-3 gap-6 h-auto">
    <CurrentTimeBlock class="row-span-2 col-span-3" />
    <template v-for="button in buttons">
      <div class="row-3">
        <Button :disabled="isLoading" @click="() => handleButtonClick(button.label)">
          <component :is="button.icon" class="fill-white p-2 h-40" />
          {{ $t(button.label) }}
        </Button>
      </div>
    </template>
    <Button @click="() => scanCard()" :disabled="isLoading">
      {{ $t('Scan Card') }}
    <span v-if="selectedTestUidHex"> (UID: {{ selectedTestUidHex }})</span>
    </Button>
    <div class="statuses">
      <div v-if="isLoading" class="col-span-6 text-blue-400 text-center mt-4">
        {{ $t('Loading...') }}
      </div>
      <div v-if="card_uid" class="col-span-6 text-green-400 text-center mt-2">
        {{ $t('Card UID:') }} {{ card_uid }}
      </div>
      <div v-if="user" class="col-span-6 text-white text-center mt-2">
        {{ $t('User:') }} <span class="font-bold">{{ user.card_name + ' ' + user.user_fullname || user.card_number }}</span>
      </div>
      <div v-if="leaveResult" class="col-span-6 text-green-400 text-center mt-2">
        <template v-if="selectedTestUidHex">
          {{ selectedTestUidHex }}
        </template>
        {{ selectedTestUidHex ? ' ' : '' }}{{ leaveResult }}
      </div>
      <div v-if="errorMsg" class="col-span-6 text-red-400 text-center mt-2">
        {{ $t('Error:') }} {{ errorMsg }}
      </div>
    </div>
    <div class="col-span-6 flex gap-2 mt-2">
      <Button
          v-for="(uid, idx) in testUids"
          :key="idx"
          :disabled="isLoading"
          :class="getCardButtonClass(uid, idx)"
          @click="() => selectTestCard(uid)">
        {{ $t('Test Card') }} {{ idx + 1 }}
      </Button>
    </div>
  </div>
</template>
