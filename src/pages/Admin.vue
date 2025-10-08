<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface CardDto {
  id: number;
  card_number: string;
  card_name?: string | null;
  user_fullname?: string | null;
  user_id?: string | null;
  is_present: boolean;
}

const cards = ref<CardDto[]>([]);
const errorMsg = ref<string | null>(null);
const isLoading = ref(false);
const editingCardId = ref<number | null>(null);
const editName = ref("");
const editFullname = ref("");
const editUserId = ref("");

async function fetchCards() {
  isLoading.value = true;
  errorMsg.value = null;
  try {
    cards.value = await invoke("get_all_cards");
  } catch (e: any) {
    errorMsg.value = e?.toString() || "Unknown error";
  } finally {
    isLoading.value = false;
  }
}

function startEdit(card: CardDto) {
  editingCardId.value = card.id;
  editName.value = card.card_name || "";
  editFullname.value = card.user_fullname || "";
  editUserId.value = card.user_id || "";
}

async function saveEdit(cardId: number) {
  isLoading.value = true;
  errorMsg.value = null;
  try {
    await invoke("update_card_info", {
      cardId,
      cardName: editName.value,
      userFullname: editFullname.value,
      userId: editUserId.value,
    });
    editingCardId.value = null;
    await fetchCards();
  } catch (e: any) {
    errorMsg.value = e?.toString() || "Unknown error";
  } finally {
    isLoading.value = false;
  }
}

function cancelEdit() {
  editingCardId.value = null;
}

async function deleteCard(cardId: number) {
  isLoading.value = true;
  errorMsg.value = null;
  try {
  await invoke('delete_card', { cardId });
    await fetchCards();
  } catch (e: any) {
    errorMsg.value = e?.toString() || 'Unknown error';
  } finally {
    isLoading.value = false;
  }
}

onMounted(fetchCards);
</script>

<template>
  <div>
    <h2 class="text-xl font-bold mb-4">{{ $t("Admin: Edit Card/User Info") }}</h2>
    <div v-if="errorMsg" class="text-red-400 mb-2">{{ errorMsg }}</div>
    <table class="w-full border mb-6">
      <thead>
        <tr class="bg-gray-700 text-white">
          <th class="p-2 text-left">{{ $t("Card Number") }}</th>
          <th class="p-2 text-left">{{ $t("User") }}</th>
          <th class="p-2 text-left">{{ $t("User ID") }}</th>
          <th class="p-2 text-left">{{ $t("Actions") }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="card in cards" :key="card.id">
          <td class="p-2">{{ card.card_number }}</td>
          <td class="p-2">
            <template v-if="editingCardId === card.id">
              <input v-model="editFullname" class="border px-1" />
            </template>
            <template v-else>{{ card.card_name }} {{ card.user_fullname }}</template>
          </td>
          <td class="p-2">
            <template v-if="editingCardId === card.id">
              <input v-model="editUserId" class="border px-1" />
            </template>
            <template v-else>{{ card.user_id }}</template>
          </td>
          <td class="p-2 flex gap-2">
            <template v-if="editingCardId === card.id">
              <button class="bg-green-600 text-white px-2 py-1 rounded mr-2" @click="saveEdit(card.id)" :disabled="isLoading">{{ $t("Save") }}</button>
              <button class="bg-gray-400 text-black px-2 py-1 rounded mr-2" @click="cancelEdit" :disabled="isLoading">{{ $t("Cancel") }}</button>
            </template>
            <template v-else>
              <button class="bg-blue-600 text-white px-2 py-1 rounded mr-2" @click="startEdit(card)">{{ $t("Edit") }}</button>
              <button class="bg-red-600 text-white px-2 py-1 rounded" @click="deleteCard(card.id)" :disabled="isLoading">{{ $t("Delete") }}</button>
            </template>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
