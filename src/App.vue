<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const excelView = ref<string>(JSON.parse("{}"));
const excel = ref<File>();

async function convert() {
  await invoke("convert", {});
}

async function viewFile() {
  excelView.value = await invoke("view_file", {});
}
</script>

<template>
  <main class="">
    <h1>Excel to XML Converter</h1>
    <form action="" method="post">
      <input type="file" name="" id="file-uploader" accept=".xlsx;.csv"/>
      <div>
        <button type="submit" >Convert</button>
        <button type="button">View File</button>
      </div>
    </form>
  </main>
</template>