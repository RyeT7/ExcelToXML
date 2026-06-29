<script setup lang="ts">
import { useConverter } from "../../composables/useConverter";

const {
  converting,
  conversionError,
  conversionSuccess,
  runConvert,
  downloadXml,
  tin,
  invoiceNumberColumn,
  goodServiceIdentifierColumn,
  tableData,
} = useConverter();
</script>

<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-100 mb-1">Convert &amp; Download</h2>
    <p class="text-sm text-gray-400 mb-6">Review your settings, then generate and save the XML.</p>

    <dl class="grid grid-cols-[auto_1fr] gap-x-6 gap-y-2 text-sm mb-6 max-w-md">
      <dt class="text-gray-400">TIN</dt>
      <dd class="font-medium text-gray-100">{{ tin || "—" }}</dd>
      <dt class="text-gray-400">Invoice number column</dt>
      <dd class="font-medium text-gray-100">{{ invoiceNumberColumn || "—" }}</dd>
      <dt class="text-gray-400">Good/service column</dt>
      <dd class="font-medium text-gray-100">{{ goodServiceIdentifierColumn || "—" }}</dd>
      <dt class="text-gray-400">Rows</dt>
      <dd class="font-medium text-gray-100">{{ tableData.length }}</dd>
    </dl>

    <div class="flex gap-3 items-center">
      <button
        @click="runConvert"
        :disabled="converting"
        class="px-5 py-2 rounded-lg bg-blue-600 text-white font-medium transition-colors hover:bg-blue-700 disabled:bg-gray-700 disabled:text-gray-500 disabled:cursor-not-allowed"
      >
        {{ converting ? "Converting…" : conversionSuccess ? "Re-convert" : "Convert to XML" }}
      </button>
      <button
        v-if="conversionSuccess"
        @click="downloadXml"
        class="px-5 py-2 rounded-lg bg-green-600 text-white font-medium transition-colors hover:bg-green-700"
      >
        Download XML
      </button>
    </div>

    <p v-if="conversionError" class="text-red-400 mt-3">{{ conversionError }}</p>
    <p v-if="conversionSuccess" class="text-green-400 mt-3">
      Conversion successful! Click “Download XML” to choose where to save the file.
    </p>
  </div>
</template>
