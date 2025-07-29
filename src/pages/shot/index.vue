<script setup lang="ts">
import type { Canvas } from 'fabric'
import { getAllWebviewWindows } from '@tauri-apps/api/webviewWindow'
import { Button } from '@/components/ui/button'
import { useDocumentFocus } from '@/composables/useDocumentFocus'
import FabricCanvas from './components/FabricCanvas.vue'
import Toolbar from './components/toolbar/Toolbar.vue'

const canvas = ref<Canvas>()

const isFocused = useDocumentFocus()

const keys = useMagicKeys()
const esc = keys.Escape
watch(esc, (v) => {
  if (v)
    closeWindow()
})

async function closeWindow() {
  const windows = await getAllWebviewWindows()
  windows.forEach((window) => {
    if (window.label.startsWith('monitor-')) {
      window.close()
    }
  })
}
</script>

<template>
  <div>
    <FabricCanvas @ready="canvas = $event" />

    <Toolbar v-if="canvas" :canvas="canvas" class="fixed bottom-10 left-10" />

    <div
      v-show="!isFocused"
      class="mask fixed inset-0 z-0  bg-gray-50/45 backdrop-blur-[1px]"
    />

    <div class="fixed bottom-200 right-200 z-50">
      <Button variant="destructive" @click="closeWindow">
        xxx
      </Button>
    </div>
  </div>
</template>

<style scoped>

</style>

<style>
html,
body {
  background-color: transparent;
  overflow: hidden;
}
</style>
