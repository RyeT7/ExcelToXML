import { invoke } from "@tauri-apps/api/core";

export interface TagDTO {
    literal: string;
    hierarchical: string;
}

export interface TagMapping {
    literal: string;
    hierarchical: string;
    mappedColumn: string | null;
    defaultValue: string;
}

export async function getHeaders(): Promise<TagDTO[]> {
    return await invoke<TagDTO[]>("view_headers", {});
}

export async function mapHeaders(sessionId: string, tagMappings: TagMapping[]): Promise<void> {
    // Convert camelCase to snake_case for backend
    const mappings = tagMappings.map(m => ({
        literal: m.literal,
        hierarchical: m.hierarchical,
        mapped_column: m.mappedColumn,
        default_value: m.defaultValue,
    }));

    return await invoke<void>("map_headers", {
        sessionId: sessionId,
        tagMappings: mappings,
    });
}