
<script setup lang="ts"></script>

<style>
.tab-selection svg {
  width: 100% !important;
  max-width: 100px !important;
}
</style>

<template>
  <button
    :class="[
      'font-bold py-2 px-4 rounded transition-colors duration-300 flex flex-col items-center justify-center gap-2 w-full tab-selection',
      $attrs.class || 'bg-radial-[at_0%_0%] from-slate-600 from-20% to-slate-700 to-80% text-white hover:from-slate-500 hover:to-slate-600',
    ]"
    v-bind="$attrs"
    @click="$emit('click')"
  >
    <slot>Button</slot>
  </button>
</template>
