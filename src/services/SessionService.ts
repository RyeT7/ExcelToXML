import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

export const sessionId = ref<string | null>(null);
let sessionPromise: Promise<string> | null = null;

export async function createSession(): Promise<string> {
  if (sessionId.value) {
    return sessionId.value;
  }

  if (!sessionPromise) {
    sessionPromise = invoke<string>("create_session", {})
      .then((id: string) => {
        sessionId.value = id;
        return id;
      })
      .catch((error: string | null) => {
        sessionPromise = null;
        throw error;
      });
  }

  return sessionPromise;
}