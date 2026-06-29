<script setup lang="ts">
import type { Step } from "../composables/useConverter";

defineProps<{
  steps: Step[];
  current: number;
}>();
</script>

<template>
  <nav class="flex items-center justify-center">
    <template v-for="(step, index) in steps" :key="step.id">
      <div class="flex items-center gap-2">
        <div
          class="w-9 h-9 rounded-full flex items-center justify-center text-sm font-semibold border-2 transition-colors"
          :class="
            current === step.id
              ? 'bg-blue-600 border-blue-600 text-white'
              : current > step.id
                ? 'bg-blue-500/20 border-blue-500 text-blue-400'
                : 'bg-gray-800 border-gray-600 text-gray-500'
          "
        >
          <span v-if="current > step.id">&checkmark;</span>
          <span v-else>{{ step.id }}</span>
        </div>
        <span
          class="text-sm font-medium"
          :class="current >= step.id ? 'text-gray-100' : 'text-gray-500'"
        >
          {{ step.label }}
        </span>
      </div>
      <div
        v-if="index < steps.length - 1"
        class="w-12 h-0.5 mx-3"
        :class="current > step.id ? 'bg-blue-500' : 'bg-gray-600'"
      ></div>
    </template>
  </nav>
</template>
