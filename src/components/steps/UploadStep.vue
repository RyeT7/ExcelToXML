<script setup lang="ts">
import { useConverter } from "../../composables/useConverter";

const { uploaded, uploading, uploadError, headers, tableData, uploadFile } = useConverter();
</script>

<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-100 mb-1">Upload Excel File</h2>
    <p class="text-sm text-gray-400 mb-4">
      Select the spreadsheet (.xlsx, .xls, or .csv) containing your invoice data.
    </p>

    <button
      @click="uploadFile"
      :disabled="uploading"
      class="px-5 py-2 rounded-lg bg-blue-600 text-white font-medium transition-colors hover:bg-blue-700 disabled:bg-gray-700 disabled:text-gray-500 disabled:cursor-not-allowed"
    >
      {{ uploading ? "Uploading…" : uploaded ? "Choose a different file" : "Choose file" }}
    </button>

    <p v-if="uploadError" class="text-red-400 mt-3">{{ uploadError }}</p>

    <div v-if="uploaded" class="mt-6">
      <p class="text-sm text-green-400 mb-2">
        File loaded — {{ tableData.length }} rows, {{ headers.length }} columns.
      </p>
      <div class="overflow-auto max-h-80 border border-gray-700 rounded-lg">
        <table class="min-w-full text-sm">
          <thead class="bg-gray-700 sticky top-0">
            <tr>
              <th
                v-for="header in headers"
                :key="header"
                class="text-left font-semibold text-gray-300 px-3 py-2 border-b border-gray-700 whitespace-nowrap"
              >
                {{ header }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, i) in tableData" :key="i" class="text-gray-300 even:bg-gray-700/30">
              <td
                v-for="header in headers"
                :key="header"
                class="px-3 py-1.5 border-b border-gray-700/50 whitespace-nowrap"
              >
                {{ row[header] }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
