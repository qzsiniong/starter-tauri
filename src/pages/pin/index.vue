<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'

import { LogicalSize } from '@tauri-apps/api/dpi'
import { getAllWebviewWindows, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { throttle } from 'lodash-es'
import { XIcon } from 'lucide-vue-next'
import { commands } from '@/bindings'
import { useWebviewWindowInitialSize } from '@/composables/useWebviewWindowInitialSize'
import { useContextMenu } from './composables/useContextMenu'

const props = defineProps({
  path: String,
})

// 获取当前窗口实例
const currentWindow = getCurrentWebviewWindow()

const initialSize = useWebviewWindowInitialSize()

definePage({
  meta: {
    layout: 'none',
  },
  props(to) {
    return { path: to.query.path }
  },
})

const imgSrc = ref(props.path ? convertFileSrc(props.path) : undefined)
const shadowing = ref(true)

async function close() {
  await currentWindow.close()
}
async function closeAll() {
  let windows = await getAllWebviewWindows()

  windows = windows
    .filter(w => w.label.startsWith('pin-'))

  windows.forEach(w => w.close())
}

useContextMenu({
  close,
  closeAll,
  toggleShadow() {
    shadowing.value = !shadowing.value
  },
  zoom(ratio) {
    let { width, height } = initialSize.value!
    width = width * ratio
    height = height * ratio

    const size = new LogicalSize(width, height)
    console.log('🥢 size', size)
    currentWindow.setSize(size)
  },
}, () => ({
  shadowing: shadowing.value,
}))

async function handleZoom(deltaY: number) {
  // 发送缩放请求到 Rust 后端
  await commands.zoomWindow(initialSize.value!, deltaY)
}

const throttledHandleZoom = throttle(handleZoom, 1)

useEventListener('wheel', async (e) => {
  e.preventDefault() // 阻止页面默认滚动（如果需要）

  throttledHandleZoom(e.deltaY)
})
</script>

<template>
  <div
    data-tauri-drag-region
    class="wrapper absolute overflow-hidden size-full border-3 border-blue-500/75"
    :class="[shadowing ? '' : 'border-transparent']"
  >
    <img data-tauri-drag-region :src="imgSrc" class="w-full">

    <div data-tauri-drag-region class="absolute top-2 right-2 bg-white ring-2 ring-red-600">
      label:{{ currentWindow.label }}
    </div>
    <button class="close hidden absolute top-2 left-2 hover:bg-red-500" @click="close">
      <XIcon />
    </button>
  </div>
</template>

<style>
body {
  background-color: transparent;
  overflow: hidden;
}
/* 禁止选中 */
* {
  @apply select-none;
}
</style>

<style scoped lang="css">
.wrapper:hover {
  .close {
    display: block;
  }
}
</style>
