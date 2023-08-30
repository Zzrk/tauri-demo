<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow } from '@tauri-apps/api/window';

function openWindowByCommand() {
  invoke("open_external_window")
}

function createWindow() {
  const webview = new WebviewWindow('external', {
    url: 'https://tauri.app/',
  })
  // since the webview window is created asynchronously,
  // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
  webview.once('tauri://created', function () {
    // webview window successfully created
  })
  webview.once('tauri://error', function () {
    // an error occurred during webview window creation
  })
}
</script>

<template>
  <div>
    <button @click="openWindowByCommand">Open Window By Command</button>
    <button @click="createWindow">Create Window</button>
  </div>
</template>
