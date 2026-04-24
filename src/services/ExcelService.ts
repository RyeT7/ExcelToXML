import { invoke } from "@tauri-apps/api/core";

export async function uploadExcelFile(
    sessionId: string,
    path: string
): Promise<void> {
    await invoke("load_excel", {
        sessionId: sessionId,
        path: path,
    });
}

export interface Table {
    headers: string[];
    data: Record<string, string>[];
}

interface ColumnarTable {
    headers: string[];
    data: Record<string, string[]>;
}

function transformColumnarToRowBased(columnar: ColumnarTable): Table {
    const rowCount = columnar.data[columnar.headers[0]]?.length || 0;
    const rows: Record<string, string>[] = [];

    for (let i = 0; i < rowCount; i++) {
        const row: Record<string, string> = {};
        for (const header of columnar.headers) {
            row[header] = columnar.data[header][i] || "";
        }
        rows.push(row);
    }

    return {
        headers: columnar.headers,
        data: rows,
    };
}

export async function viewExcelTable(sessionId: string): Promise<Table> {
    const result = await invoke<ColumnarTable>("view_excel", {
        sessionId: sessionId,
    });
    return transformColumnarToRowBased(result);
}