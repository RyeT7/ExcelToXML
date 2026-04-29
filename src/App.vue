<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { createSession, sessionId } from "./services/SessionService";
import { open } from "@tauri-apps/plugin-dialog";
import { uploadExcelFile, viewExcelTable } from "./services/ExcelService";
import { getHeaders, TagDTO, mapHeaders } from "./services/ParseService";

interface TagMapping {
  literal: string;
  hierarchical: string;
  mappedColumn: string | null;  // Excel header name or null
  defaultValue: string;
}

const appSessionId = sessionId;
const loadingSession = ref(false);
const sessionError = ref<string | null>(null);

const uploaded = ref<boolean>(false);
const uploading = ref(false);
const uploadError = ref<string | null>(null);

const headers = ref<string[]>([]);
const tableData = ref<Record<string, string>[]>([]);

const requiredTags = ref<TagDTO[]>([]);
const tagMappings = ref<TagMapping[]>([]);

onMounted(async () => {
  loadingSession.value = true;
  try {
    await createSession();
  } catch (error) {
    sessionError.value = error instanceof Error ? error.message : String(error);
  } finally {
    loadingSession.value = false;
  }
});

async function loadHeaders() {
  if (!appSessionId.value) {
    uploadError.value = "Session not initialized";
    return;
  }

  try {
    const requiredTagsData = await getHeaders();
    requiredTags.value = Array.isArray(requiredTagsData) ? requiredTagsData : [];
    console.log(requiredTags.value)
  } catch (error) {
    uploadError.value = error instanceof Error ? error.message : String(error);
  }
}

function initializeMappings() {
  tagMappings.value = requiredTags.value.map(tag => ({
    literal: tag.literal,
    hierarchical: tag.hierarchical,
    mappedColumn: null,
    defaultValue: ""
  }));
}

async function convert() {
  if (!appSessionId.value) {
    uploadError.value = "Session not initialized";
    return;
  }

  try {
    // Validate and store mappings
    await mapHeaders(appSessionId.value, tagMappings.value);

    // Then perform conversion
    await invoke("convert", {
      sessionId: appSessionId.value,
    });
  } catch (error) {
    uploadError.value = error instanceof Error ? error.message : String(error);
  }
}

async function uploadFile() {
  if (!appSessionId.value) {
    uploadError.value = "Session not initialized";
    return;
  }

  uploading.value = true;
  uploadError.value = null;

  try {
    const selected = await open({
      filters: [{ name: "Excel", extensions: ["xlsx", "xls", "csv"] }],
    });

    if (typeof selected === "string" && selected) {
      await uploadExcelFile(
        appSessionId.value,
        selected
      );
      uploaded.value = true;

      const table = await viewExcelTable(appSessionId.value);
      headers.value = table.headers;
      tableData.value = table.data;

      await loadHeaders();
      initializeMappings();
    }
  } catch (error) {
    uploadError.value = error instanceof Error ? error.message : String(error);
  } finally {
    uploading.value = false;
  }
}
</script>

<template>
  <main class="">
    <h1>Excel to XML Converter</h1>
    <p v-if="loadingSession">Initializing session...</p>
    <p v-if="sessionError" class="error">Session error: {{ sessionError }}</p>
    
    <form @submit.prevent="uploadFile">
      <div>
        <button type="button" @click="uploadFile" :disabled="uploading || loadingSession">
          {{ uploading ? "Uploading..." : "Convert" }}
        </button>
      </div>
    </form>
    
    <p v-if="uploadError" class="error">Upload error: {{ uploadError }}</p>
    
    <table v-if="uploaded" class="table-auto border-collapse border border-gray-300 mt-4">
      <thead>
        <tr>
          <th v-for="header in headers" :key="header">{{ header }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, index) in tableData" :key="index">
          <td v-for="header in headers" :key="header">{{ row[header] }}</td>
        </tr>
      </tbody>
    </table>

    <div v-if="uploaded">
      <h1>Map Headers</h1>
      <p class="text-sm text-gray-600 mb-4">Each required tag must be mapped to an Excel column or assigned a default value.</p>
      <table class="table-auto border-collapse border border-gray-300 mt-4">
        <thead>
          <tr>
            <th class="border border-gray-300 p-2">Required Tag</th>
            <th class="border border-gray-300 p-2">Map to Excel Column</th>
            <th class="border border-gray-300 p-2">Default Value</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="mapping in tagMappings" :key="mapping.literal">
            <td class="border border-gray-300 p-2">
              <span class="font-semibold">{{ mapping.hierarchical }}</span>
              <div class="text-xs text-gray-500">({{ mapping.literal }})</div>
            </td>
            <td class="border border-gray-300 p-2">
              <select v-model="mapping.mappedColumn" class="w-full border border-gray-300 p-1">
                <option value="">-- Use default value --</option>
                <option v-for="header in headers" :key="header" :value="header">
                  {{ header }}
                </option>
              </select>
            </td>
            <td class="border border-gray-300 p-2">
              <input 
                v-if="!mapping.mappedColumn"
                v-model="mapping.defaultValue" 
                type="text" 
                placeholder="Required" 
                class="w-full border border-gray-300 p-1"
                required
              />
              <span v-else class="text-gray-500">N/A</span>
            </td>
          </tr>
        </tbody>
      </table>
      <button @click="" class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
        Convert to XML
      </button>
    </div>
  </main>
</template>