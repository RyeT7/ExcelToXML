<script setup lang="ts">
import { onMounted } from "vue";
import { useConverter } from "./composables/useConverter";
import Stepper from "./components/Stepper.vue";
import UploadStep from "./components/steps/UploadStep.vue";
import MapStep from "./components/steps/MapStep.vue";
import ConvertStep from "./components/steps/ConvertStep.vue";

const { steps, currentStep, canProceed, next, back, initSession, loadingSession, sessionError } =
  useConverter();

onMounted(() => {
  initSession();
});
</script>

<template>
  <main class="min-h-screen bg-gray-900 text-gray-200 py-10 px-4">
    <div class="max-w-4xl mx-auto">
      <header class="mb-8 text-center">
        <h1 class="text-3xl font-bold text-gray-100">Excel to XML Converter</h1>
        <p class="text-gray-400 mt-1">Convert tax invoice spreadsheets into CoreTax XML</p>
      </header>

      <p v-if="loadingSession" class="text-center text-gray-400 mb-4">Initializing session…</p>
      <p v-if="sessionError" class="text-center text-red-400 mb-4">
        Session error: {{ sessionError }}
      </p>

      <Stepper :steps="steps" :current="currentStep" class="mb-8" />

      <div class="bg-gray-800 rounded-xl shadow-lg border border-gray-700 p-6">
        <Transition name="fade" mode="out-in">
          <UploadStep v-if="currentStep === 1" key="upload" />
          <MapStep v-else-if="currentStep === 2" key="map" />
          <ConvertStep v-else key="convert" />
        </Transition>
      </div>

      <div class="flex justify-between mt-6">
        <button
          v-if="currentStep > 1"
          @click="back"
          class="px-5 py-2 rounded-lg border border-gray-600 text-gray-200 font-medium transition-colors hover:bg-gray-700 active:bg-gray-600"
        >
          Back
        </button>
        <span v-else></span>

        <button
          v-if="currentStep < steps.length"
          @click="next"
          :disabled="!canProceed"
          class="px-5 py-2 rounded-lg bg-blue-600 text-white font-medium shadow-sm transition-colors hover:bg-blue-700 active:bg-blue-800 disabled:bg-gray-700 disabled:text-gray-500 disabled:cursor-not-allowed disabled:shadow-none"
        >
          Next
        </button>
      </div>
    </div>
  </main>
</template>