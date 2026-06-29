<script setup lang="ts">
import { ref, onMounted } from "vue";
import { createSession, sessionId } from "./services/SessionService";
import { open, save } from "@tauri-apps/plugin-dialog";
import { uploadExcelFile, viewExcelTable } from "./services/ExcelService";
import { getHeaders, TagDTO, mapHeaders, convert, saveXml } from "./services/ParseService";

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
const tin = ref<string>("");
const invoiceNumberColumn = ref<string>("");
const goodServiceIdentifierColumn = ref<string>("");

const converting = ref(false);
const conversionError = ref<string | null>(null);
const conversionSuccess = ref(false);

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

async function handleConvert() {
  if (!appSessionId.value) {
    conversionError.value = "Session not initialized";
    return;
  }

  if (!tin.value.trim()) {
    conversionError.value = "TIN is required";
    return;
  }

  if (!invoiceNumberColumn.value) {
    conversionError.value = "Invoice number column is required";
    return;
  }

  if (!goodServiceIdentifierColumn.value) {
    conversionError.value = "Good/service identifier column is required";
    return;
  }

  converting.value = true;
  conversionError.value = null;
  conversionSuccess.value = false;

  try {
    // Validate and store mappings
    await mapHeaders(
      appSessionId.value,
      tagMappings.value,
      invoiceNumberColumn.value,
      goodServiceIdentifierColumn.value,
    );

    // Then perform conversion
    await convert(appSessionId.value, tin.value);
    
    conversionSuccess.value = true;
  } catch (error) {
    conversionError.value = error instanceof Error ? error.message : String(error);
  } finally {
    converting.value = false;
  }
}

async function downloadXml() {
  if (!appSessionId.value) {
    conversionError.value = "Session not initialized";
    return;
  }

  try {
    // Let the user pick where to save the file.
    const path = await save({
      defaultPath: `converted-${new Date().toISOString().slice(0, 10)}.xml`,
      filters: [{ name: "XML", extensions: ["xml"] }],
    });

    // User cancelled the dialog.
    if (!path) {
      return;
    }

    await saveXml(appSessionId.value, path);
  } catch (error) {
    conversionError.value = error instanceof Error ? error.message : String(error);
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
      
      <div class="mb-4">
        <label for="tin-input" class="block text-sm font-medium mb-2">TIN (Tax Identification Number)</label>
        <input
          id="tin-input"
          v-model="tin"
          type="text"
          placeholder="Enter TIN"
          class="w-full border border-gray-300 p-2 mb-2"
        />
      </div>

      <div class="mb-4">
        <label for="invoice-number-column" class="block text-sm font-medium mb-2">Invoice Number Column</label>
        <select id="invoice-number-column" v-model="invoiceNumberColumn" class="w-full border border-gray-300 p-2 mb-2">
          <option value="">-- Select column --</option>
          <option v-for="header in headers" :key="header" :value="header">{{ header }}</option>
        </select>
      </div>

      <div class="mb-4">
        <label for="good-service-column" class="block text-sm font-medium mb-2">Good/Service Identifier Column</label>
        <select id="good-service-column" v-model="goodServiceIdentifierColumn" class="w-full border border-gray-300 p-2 mb-2">
          <option value="">-- Select column --</option>
          <option v-for="header in headers" :key="header" :value="header">{{ header }}</option>
        </select>
      </div>

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
                <option :value="null">-- Use default value --</option>
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
                placeholder="Optional (empty allowed)"
                class="w-full border border-gray-300 p-1"
              />
              <span v-else class="text-gray-500">N/A</span>
            </td>
          </tr>
        </tbody>
      </table>
      <button @click="handleConvert" :disabled="converting" class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400">
        {{ converting ? "Converting..." : "Convert to XML" }}
      </button>

      <p v-if="conversionError" class="error text-red-600 mt-2">{{ conversionError }}</p>
      <p v-if="conversionSuccess" class="text-green-600 mt-2">Conversion successful!</p>

      <button v-if="conversionSuccess" @click="downloadXml" class="mt-2 ml-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600">
        Download XML
      </button>
    </div>
  </main>
</template>