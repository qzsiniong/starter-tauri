<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { getAllWebviewWindows, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

import { XIcon } from 'lucide-vue-next'
import { useContextMenu } from './composables/useContextmenu'
import { useImgSize } from './composables/useImgSize'

const props = defineProps({
  path: String,
})

// 获取当前窗口实例
const currentWindow = getCurrentWebviewWindow()

// 配置：每次滚动的尺寸调整步长（宽/高各增加/减少 50px）
const SCALE_STEP = 10
// 窗口最小尺寸（避免无限缩小）
const MIN_WIDTH = 30
// const MIN_HEIGHT = 30
// 窗口最大尺寸（避免无限放大）
const MAX_WIDTH = 1600
// const MAX_HEIGHT = 1200

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

const imgSize = useImgSize(imgSrc.value!)
const windowSize = reactive({
  width: 0,
  height: 0,
})

const windowOriginalSize = computed(() => {
  return {
    width: imgSize.width + 3,
    height: imgSize.height + 3,
  }
})
watch(windowOriginalSize, (newVal) => {
  windowSize.width = newVal.width
  windowSize.height = newVal.height
})
watch(windowSize, (newVal) => {
  const { width, height } = newVal
  currentWindow.setSize(new LogicalSize(width, height))
})

async function close() {
  const window = getCurrentWebviewWindow()
  await window.close()
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
    let { width, height } = windowOriginalSize.value
    width = width * ratio
    height = height * ratio

    currentWindow.setSize(new LogicalSize(width, height))
  },
}, () => ({
  shadowing: shadowing.value,
}))

useEventListener('wheel', async (e) => {
  e.preventDefault() // 阻止页面默认滚动（如果需要）

  const [
    // position,
    size,
  ] = await Promise.all([
    // currentWindow.innerPosition(),
    currentWindow.size(),
  ])

  // const { x, y } = position

  // 获取当前窗口尺寸
  let { width: newWidth, height: newHeight } = size

  // 根据滚动方向调整尺寸（上滚=放大，下滚=缩小）
  if (e.deltaY < 0) { // 上滚（deltaY 为负）
    newHeight += SCALE_STEP
  }
  else { // 下滚（deltaY 为正）
    newHeight -= SCALE_STEP
  }

  if (newHeight !== Math.max(MIN_WIDTH, Math.min(newHeight, MAX_WIDTH))) {
    newHeight = Math.max(MIN_WIDTH, Math.min(newHeight, MAX_WIDTH))
  }

  // 根据宽高比计算对应的高度
  newWidth = newHeight * imgSize.ratio

  // 应用新尺寸
  await currentWindow.setSize(new LogicalSize(newWidth, newHeight))

  // const ratio = newWidth / imgSize.width

  // 计算新位置
  // const newX = x + (e.pageX - e.pageX * ratio)
  // const newY = y + (e.pageY - e.pageY * ratio)
  // await currentWindow.setPosition(new LogicalPosition(newX, newY))
  // console.log(`🍴 (${x}, ${y}) ==> (${newX}, ${newY})`);
})
</script>

<template>
  <div data-tauri-drag-region class="wrapper absolute size-full p-[3px]">
    <div class="absolute top-2 right-2 bg-white ring-2 ring-red-600">
      label:{{ currentWindow.label }}
    </div>
    <button class="close hidden absolute top-2 left-2 hover:bg-red-500" @click="close">
      <XIcon />
    </button>
    <div
      data-tauri-drag-region
      class=" ring-blue-500/75"
      :class="[shadowing ? 'ring-3' : '']"
    >
      <img
        data-tauri-drag-region
        :src="imgSrc"
        class="size-full"
      >
    </div>
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
