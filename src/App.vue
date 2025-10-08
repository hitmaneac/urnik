<script setup lang="ts">
import { ref } from "vue";
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getVersion } from "@tauri-apps/api/app";
import NetworkIcon from "./assets/network.svg";
import InternetIcon from "./assets/globe.svg";
import AdminIcon from "./assets/badge.svg";
import HomeIcon from "./assets/home.svg";
import InfoIcon from "./assets/info.svg";

const currentUserName = ref("");
onMounted(() => {
  window.addEventListener("urniknet-user", (e: any) => {
    currentUserName.value = e.detail || "";
  });
});

const appVersion = ref("unknown");
getVersion().then((v) => {
  appVersion.value = v;
});


const isConnected = ref(false);
async function checkBackend() {
  try {
    await invoke("greet", { name: "ping" });
    isConnected.value = true;
  } catch {
    isConnected.value = false;
  }
}
checkBackend();
setInterval(checkBackend, 10000);

const isInternetConnected = ref(navigator.onLine);

window.addEventListener("online", () => {
  isInternetConnected.value = true;
});
window.addEventListener("offline", () => {
  isInternetConnected.value = false;
});

const menu = ref(1);

const organization = ref("SK-Media x XPRO");
const showInfo = ref(false);

</script>

<template>
  <header class="fixed flex bg-gray-800 text-white p-2 px-4 gap-4 w-full">
    <RouterLink to="/" @click="menu = 1">
      <HomeIcon class="w-6 h-6 fill-gray-500"
      :class="{
        'fill-white': menu === 1
      }"/>
    </RouterLink>
    <RouterLink to="/overview" @click="menu = 2">
      <InfoIcon class="w-6 h-6 fill-gray-500"
      :class="{
        'fill-white': menu === 2
      }"/>
    </RouterLink>
    <RouterLink to="/admin" @click="menu = 3">
    <AdminIcon class="w-6 h-6 fill-gray-500"
    :class="{
        'fill-white': menu === 3
      }"/>
    </RouterLink>

    &nbsp; &nbsp;
    <InternetIcon
      class="w-6 h-6"
      :class="{
        'fill-slate-200': isInternetConnected,
        'fill-red-800': !isInternetConnected,
      }"
    />
    <NetworkIcon
      class="w-6 h-6"
      :class="{ 'fill-slate-200': isConnected, 'fill-red-800': !isConnected }"
    />
    
    <div class="flex-auto flex justify-center">
      <span v-if="currentUserName">{{ $t("message.welcome") }}<template v-if="currentUserName">{{ currentUserName }}</template></span>
    </div>
    <div class="font-bold">{{ organization }}</div>
    <div>
      <select
        class="bg-gray-700 text-sm rounded-sm p-1 h-6"
        v-model="$i18n.locale"
      >
        <option
          v-for="locale in $i18n.availableLocales"
          :key="`locale-${locale}`"
          :value="locale"
        >
          {{ locale }}
        </option>
      </select>
    </div>
    <InfoIcon class="w-6 h-6 fill-white" @click="showInfo = !showInfo" />
  </header>
  <main class="p-6 pt-18 min-h-screen">
    <RouterView />
  </main>
  <div
    id="blackout"
    class="fixed bg-gray-400 opacity-70 min-h-full w-full top-0 left-0 z-50"
    :class="{ hidden: !showInfo }"
  ></div>
  <div
    class="fixed top-0 left-0 w-full min-h-full z-51 flex items-center justify-center"
    :class="{ hidden: !showInfo }"
    @click="showInfo = false"
  >
    <div
      class="w-2/3 h-2/3 flex flex-col gap-2 p-6 bg-slate-700 rounded-2xl border-4 border-slate-600 text-white items-center justify-center"
    >
      <img src="./assets/UrnikNET.png" class="h-1/6 mx-auto mb-2" />
      <p class="text-center">{{ $t("Version") }} {{ appVersion }}</p>
    </div>
  </div>
</template>
