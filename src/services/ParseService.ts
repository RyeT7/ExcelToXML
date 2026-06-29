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

export async function mapHeaders(
    sessionId: string,
    tagMappings: TagMapping[],
    invoiceNumberColumn: string,
    goodServiceIdentifierColumn: string,
): Promise<void> {
    // Build the TagMappingsDTO the backend expects: a map keyed by the
    // hierarchical tag path, plus the two grouping columns.
    const tag_mappings: Record<string, { mapped_column: string | null; default_value: string | null }> = {};
    for (const m of tagMappings) {
        tag_mappings[m.hierarchical] = {
            mapped_column: m.mappedColumn,
            default_value: m.defaultValue,
        };
    }

    return await invoke<void>("map_headers", {
        sessionId: sessionId,
        tagMappings: {
            tag_mappings,
            invoice_number_column: invoiceNumberColumn,
            good_service_identifier_column: goodServiceIdentifierColumn,
        },
    });
}

export async function convert(sessionId: string, tin: string): Promise<void> {
    return await invoke<void>("convert", {
        sessionId: sessionId,
        tin: tin,
    });
}

export async function getXml(sessionId: string): Promise<string> {
    return await invoke<string>("get_xml", {
        sessionId: sessionId,
    });
}

export async function saveXml(sessionId: string, path: string): Promise<void> {
    return await invoke<void>("save_xml", {
        sessionId: sessionId,
        path: path,
    });
}