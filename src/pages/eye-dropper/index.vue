<script setup lang="ts">
import type {
  Options as SendNotificationOptions,
} from '@tauri-apps/plugin-notification'
import { getAllWebviewWindows } from '@tauri-apps/api/webviewWindow'
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification'
import { chunk } from 'lodash-es'
import { writeText } from 'tauri-plugin-clipboard-api'
import { commands } from '@/bindings'
import { rgbToHex } from './util'

const keys = useMagicKeys()
const esc = keys.Escape
watch(esc, (v) => {
  if (v) {
    closeWindow()
  }
})

async function closeWindow() {
  const windows = await getAllWebviewWindows()
  windows.forEach((window) => {
    if (window.label.startsWith('eye-dropper-')) {
      window.close()
    }
  })
}

const pickedData = ref<number[]>()
const position = reactive({ x: 0, y: 0 })

const computedData = computed(() => {
  const data = pickedData.value!

  const d = chunk(data, 4).map((item) => {
    return {
      isCenter: false,
      color: rgbToHex({
        r: item[0],
        g: item[1],
        b: item[2],
        a: item[3],
      }),
    }
  })
  const width = Math.sqrt(d.length)
  const dd = chunk(d, width)
  if (dd.length % 2 === 0) {
    dd.pop()
    dd.forEach(row => row.pop())
  }
  const c_i = (dd.length - 1) / 2
  dd[c_i][c_i].isCenter = true
  return {
    list: dd,
    color: dd[c_i][c_i].color,
  }
})

const cls = computed(() => {
  const cls = [] as string[]
  if (position.x > window.innerWidth - 200) {
    cls.push('-ml-[100%]')
  }
  if (position.y > window.innerHeight - 200) {
    cls.push('-mt-[100%]')
  }

  return cls.join(' ')
})

useEventListener('mousemove', async (e) => {
  position.x = e.clientX
  position.y = e.clientY

  const res = await commands.getColor()
  if (res.status === 'ok') {
    pickedData.value = res.data
  }
  else {
    console.error('💛 s.error', res.error)
  }
})

useEventListener('click', async () => {
  console.log(`%c${computedData.value.color}`, `background: ${computedData.value.color}`)

  await writeText(computedData.value.color)

  await notification({
    title: `取色成功`,
    body: `已复制至剪贴板: ${computedData.value.color}`,
  })

  closeWindow()
})

async function notification(options: SendNotificationOptions | string) {
  // 你有发送通知的权限吗？
  let permissionGranted = await isPermissionGranted()

  // 如果没有，我们需要请求它
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === 'granted'
  }

  // 一旦获得许可，我们就可以发送通知
  if (permissionGranted) {
    sendNotification(options)
  }
}
</script>

<template>
  <div class="fixed" :style="{ left: `${position.x}px`, top: `${position.y}px` }">
    <div class="size-32">
      <div
        class="cursor-none size-full rounded-full border-4 border-white shadow-lg overflow-hidden"
        :class="cls"
        :style1="{ marginLeft: '-50%', marginTop: '-50%' }"
      >
        <div class="size-full flex flex-col gap-[1px] bg-gray-500">
          <div v-for="(row, i) in computedData.list" :key="i" class="flex flex-1  gap-[1px]">
            <span
              v-for="(col, j) in row" :key="j"
              class="flex-1"
              :class="[col.isCenter && 'ring-[1px] ring-white']"
              :style="{ backgroundColor: col.color }"
            />
          </div>
        </div>
      </div>
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
* {
  @apply cursor-crosshair;
}
</style>
