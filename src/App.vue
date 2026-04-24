<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { createSession, sessionId } from "./services/SessionService";
import { open } from "@tauri-apps/plugin-dialog";
import { uploadExcelFile, viewExcelTable } from "./services/ExcelService";
import { app } from "@tauri-apps/api";

const appSessionId = sessionId;
const loadingSession = ref(false);
const sessionError = ref<string | null>(null);

const uploaded = ref<boolean>(false);
const uploading = ref(false);
const uploadError = ref<string | null>(null);

const headers = ref<string[]>([]);
const tableData = ref<Record<string, string>[]>([]);

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

async function convert() {
  await invoke("convert", {});
}

async function getHeaders() {
  if (!appSessionId.value) {
    uploadError.value = "Session not initialized";
    return;
  }

  try {
    const headersData = await invoke("get_headers", { sessionId: appSessionId.value });
    headers.value = Array.isArray(headersData) ? headersData : [];
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
  </main>
</template>