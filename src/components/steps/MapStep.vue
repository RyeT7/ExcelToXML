<script setup lang="ts">
import { computed } from "vue";
import { useConverter, type TagMapping } from "../../composables/useConverter";

const { headers, tagMappings, tin, invoiceNumberColumn, goodServiceIdentifierColumn } =
  useConverter();

interface TreeNode {
  name: string;
  children: TreeNode[];
  childMap: Map<string, TreeNode>;
  mapping: TagMapping | null; // set only on mappable leaves
}

interface TreeRow {
  path: string; // unique key
  prefix: string; // tree branch art, e.g. "│   ├── "
  label: string;
  mapping: TagMapping | null;
}

// Build a nested tree from every tag's hierarchical path, then flatten it into
// rows carrying `tree`-style connector art so the XML nesting is visible.
const treeRows = computed<TreeRow[]>(() => {
  const root: TreeNode = { name: "", children: [], childMap: new Map(), mapping: null };

  for (const mapping of tagMappings.value) {
    const segments = mapping.hierarchical.split(".");
    let node = root;

    segments.forEach((segment, idx) => {
      let child = node.childMap.get(segment);
      if (!child) {
        child = { name: segment, children: [], childMap: new Map(), mapping: null };
        node.childMap.set(segment, child);
        node.children.push(child);
      }
      node = child;
      if (idx === segments.length - 1) {
        node.mapping = mapping;
      }
    });
  }

  const rows: TreeRow[] = [];

  const walk = (node: TreeNode, parentPrefix: string, ancestorPath: string, isLast: boolean) => {
    const connector = isLast ? "└── " : "├── ";
    const path = ancestorPath ? `${ancestorPath}.${node.name}` : node.name;

    rows.push({ path, prefix: parentPrefix + connector, label: node.name, mapping: node.mapping });

    const childPrefix = parentPrefix + (isLast ? "    " : "│   ");
    node.children.forEach((child, i) =>
      walk(child, childPrefix, path, i === node.children.length - 1),
    );
  };

  // Top-level node(s) (TaxInvoiceBulk) render with no connector.
  root.children.forEach((node) => {
    rows.push({ path: node.name, prefix: "", label: node.name, mapping: node.mapping });
    node.children.forEach((child, i) =>
      walk(child, "", node.name, i === node.children.length - 1),
    );
  });

  return rows;
});
</script>

<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-100 mb-1">Map Headers</h2>
    <p class="text-sm text-gray-400 mb-6">
      Choose the grouping columns and map each required tag to an Excel column or a default value.
      Tags marked <span class="text-blue-300">Computed</span> are derived from the others during
      conversion.
    </p>

    <div class="grid sm:grid-cols-3 gap-4 mb-6">
      <div>
        <label for="tin-input" class="block text-sm font-medium text-gray-300 mb-1">
          TIN (Tax Identification Number)
        </label>
        <input
          id="tin-input"
          v-model="tin"
          type="text"
          placeholder="Enter TIN"
          class="w-full border border-gray-600 bg-gray-900 text-gray-100 rounded-lg p-2 transition-colors focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/30"
        />
      </div>

      <div>
        <label for="invoice-number-column" class="block text-sm font-medium text-gray-300 mb-1">
          Invoice Number Column
        </label>
        <select
          id="invoice-number-column"
          v-model="invoiceNumberColumn"
          class="w-full border border-gray-600 bg-gray-900 text-gray-100 rounded-lg p-2 transition-colors focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/30"
        >
          <option value="">-- Select column --</option>
          <option v-for="header in headers" :key="header" :value="header">{{ header }}</option>
        </select>
      </div>

      <div>
        <label for="good-service-column" class="block text-sm font-medium text-gray-300 mb-1">
          Good/Service Identifier Column
        </label>
        <select
          id="good-service-column"
          v-model="goodServiceIdentifierColumn"
          class="w-full border border-gray-600 bg-gray-900 text-gray-100 rounded-lg p-2 transition-colors focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/30"
        >
          <option value="">-- Select column --</option>
          <option v-for="header in headers" :key="header" :value="header">{{ header }}</option>
        </select>
      </div>
    </div>

    <div class="overflow-auto max-h-96 border border-gray-700 rounded-lg">
      <table class="min-w-full text-sm">
        <thead class="bg-gray-700 sticky top-0 z-10">
          <tr>
            <th class="text-left font-semibold text-gray-300 px-3 py-2 border-b border-gray-700">
              Tag Hierarchy
            </th>
            <th class="text-left font-semibold text-gray-300 px-3 py-2 border-b border-gray-700">
              Map to Excel Column
            </th>
            <th class="text-left font-semibold text-gray-300 px-3 py-2 border-b border-gray-700">
              Default Value
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="row in treeRows"
            :key="row.path"
            :class="row.mapping ? 'hover:bg-gray-700/40' : ''"
          >
            <td class="px-3 py-1.5 border-b border-gray-700/50 align-middle">
              <span class="font-mono whitespace-pre text-gray-600 select-none">{{
                row.prefix
              }}</span
              ><span
                class="font-mono"
                :class="row.mapping ? 'text-gray-100' : 'font-semibold text-gray-400'"
                >{{ row.label }}</span
              >
            </td>
            <!-- Derived tags are computed during conversion, so they show
                 their formula in place of the two inputs. -->
            <td
              v-if="row.mapping?.derived"
              colspan="2"
              class="px-3 py-1.5 border-b border-gray-700/50 align-middle"
            >
              <span class="inline-flex items-center gap-2">
                <span
                  class="text-xs font-medium px-1.5 py-0.5 rounded bg-blue-500/15 text-blue-300 border border-blue-500/30"
                >
                  Computed
                </span>
                <code class="font-mono text-xs text-gray-400">{{ row.mapping.formula }}</code>
              </span>
            </td>
            <template v-else>
              <td class="px-3 py-1.5 border-b border-gray-700/50 align-middle">
                <select
                  v-if="row.mapping"
                  v-model="row.mapping.mappedColumn"
                  class="w-full border border-gray-600 bg-gray-900 text-gray-100 rounded p-1 transition-colors focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/30"
                >
                  <option :value="null">-- Use default value --</option>
                  <option v-for="header in headers" :key="header" :value="header">
                    {{ header }}
                  </option>
                </select>
              </td>
              <td class="px-3 py-1.5 border-b border-gray-700/50 align-middle">
                <template v-if="row.mapping">
                  <input
                    v-if="!row.mapping.mappedColumn"
                    v-model="row.mapping.defaultValue"
                    type="text"
                    placeholder="Optional (empty allowed)"
                    class="w-full border border-gray-600 bg-gray-900 text-gray-100 rounded p-1 transition-colors focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/30"
                  />
                  <span v-else class="text-gray-500">N/A</span>
                </template>
              </td>
            </template>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
