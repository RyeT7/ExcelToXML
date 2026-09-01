import { ref, computed } from "vue";
import { createSession, sessionId } from "../services/SessionService";
import { open, save } from "@tauri-apps/plugin-dialog";
import { uploadExcelFile, viewExcelTable } from "../services/ExcelService";
import { getHeaders, mapHeaders, convert, saveXml } from "../services/ParseService";

export interface TagMapping {
  literal: string;
  hierarchical: string;
  derived: boolean; // computed during conversion; not user-editable
  formula: string | null; // shown in place of the inputs on derived tags
  mappedColumn: string | null; // Excel header name or null (use default value)
  defaultValue: string;
}

export interface Step {
  id: number;
  label: string;
}

const steps: Step[] = [
  { id: 1, label: "Upload" },
  { id: 2, label: "Map" },
  { id: 3, label: "Convert" },
];

// --- shared reactive state (module-level singleton) ---
const currentStep = ref(1);

const loadingSession = ref(false);
const sessionError = ref<string | null>(null);
let sessionInitialized = false;

const uploaded = ref(false);
const uploading = ref(false);
const uploadError = ref<string | null>(null);

const headers = ref<string[]>([]);
const tableData = ref<Record<string, string>[]>([]);

const tagMappings = ref<TagMapping[]>([]);
const tin = ref("");
const invoiceNumberColumn = ref("");
const goodServiceIdentifierColumn = ref("");

const converting = ref(false);
const conversionError = ref<string | null>(null);
const conversionSuccess = ref(false);

function errMsg(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

async function initSession() {
  if (sessionInitialized) return;
  sessionInitialized = true;

  loadingSession.value = true;
  try {
    await createSession();
  } catch (error) {
    sessionError.value = errMsg(error);
  } finally {
    loadingSession.value = false;
  }
}

async function uploadFile() {
  if (!sessionId.value) {
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
      await uploadExcelFile(sessionId.value, selected);

      const table = await viewExcelTable(sessionId.value);
      headers.value = table.headers;
      tableData.value = table.data;

      const requiredTags = await getHeaders();
      tagMappings.value = (Array.isArray(requiredTags) ? requiredTags : []).map((tag) => ({
        literal: tag.literal,
        hierarchical: tag.hierarchical,
        derived: tag.derived,
        formula: tag.formula,
        mappedColumn: null,
        defaultValue: "",
      }));

      // Re-uploading invalidates any previous conversion / selections.
      invoiceNumberColumn.value = "";
      goodServiceIdentifierColumn.value = "";
      conversionSuccess.value = false;
      conversionError.value = null;
      uploaded.value = true;
    }
  } catch (error) {
    uploadError.value = errMsg(error);
  } finally {
    uploading.value = false;
  }
}

async function runConvert() {
  if (!sessionId.value) {
    conversionError.value = "Session not initialized";
    return;
  }

  converting.value = true;
  conversionError.value = null;
  conversionSuccess.value = false;

  try {
    await mapHeaders(
      sessionId.value,
      tagMappings.value,
      invoiceNumberColumn.value,
      goodServiceIdentifierColumn.value,
    );
    await convert(sessionId.value, tin.value);
    conversionSuccess.value = true;
  } catch (error) {
    conversionError.value = errMsg(error);
  } finally {
    converting.value = false;
  }
}

async function downloadXml() {
  if (!sessionId.value) {
    conversionError.value = "Session not initialized";
    return;
  }

  try {
    const path = await save({
      defaultPath: `converted-${new Date().toISOString().slice(0, 10)}.xml`,
      filters: [{ name: "XML", extensions: ["xml"] }],
    });

    if (!path) return; // user cancelled

    await saveXml(sessionId.value, path);
  } catch (error) {
    conversionError.value = errMsg(error);
  }
}

const mappingsValid = computed(
  () =>
    tin.value.trim() !== "" &&
    invoiceNumberColumn.value !== "" &&
    goodServiceIdentifierColumn.value !== "",
);

const canProceed = computed(() => {
  if (currentStep.value === 1) return uploaded.value;
  if (currentStep.value === 2) return mappingsValid.value;
  return true;
});

function next() {
  if (canProceed.value && currentStep.value < steps.length) {
    currentStep.value++;
  }
}

function back() {
  if (currentStep.value > 1) {
    currentStep.value--;
  }
}

export function useConverter() {
  return {
    steps,
    currentStep,
    loadingSession,
    sessionError,
    uploaded,
    uploading,
    uploadError,
    headers,
    tableData,
    tagMappings,
    tin,
    invoiceNumberColumn,
    goodServiceIdentifierColumn,
    converting,
    conversionError,
    conversionSuccess,
    sessionId,
    mappingsValid,
    canProceed,
    initSession,
    uploadFile,
    runConvert,
    downloadXml,
    next,
    back,
  };
}
