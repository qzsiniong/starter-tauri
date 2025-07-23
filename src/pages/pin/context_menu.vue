<script setup lang="ts">
import type { ContextMenuOptions } from './composables/useContextMenuItems'
import { emitTo } from '@tauri-apps/api/event'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import {
  ContextMenu,
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuTrigger,
} from '@/components/ui/context-menu'
import { useContextMenuItems } from './composables/useContextMenuItems'

const props = defineProps<{
  label: string
  x: number
  y: number
  options: ContextMenuOptions
}>()

definePage({
  meta: {
    layout: 'none',
  },
  props(to) {
    const { label, x, y, options } = to.query
    return {
      label,
      x: Number(x),
      y: Number(y),
      options: JSON.parse(options as string),
    }
  },
})

const trigger = useTemplateRef('trigger')

const items = useContextMenuItems(async (event) => {
  await emitTo(props.label!, event.type, (event as any).payload)
  close()
}, props.options)

function close() {
  const current = getCurrentWebviewWindow()
  current.close()
}

function handleOpenChange(open: boolean) {
  if (!open) {
    close()
  }
}

function openContextMenu() {
  // 创建右键菜单事件
  const contextMenuEvent = new MouseEvent('contextmenu', {
    bubbles: true, // 是否冒泡
    cancelable: true, // 是否可取消
    view: window, // 关联的视图
    button: 2, // 2 表示右键 (0=左键, 1=中键, 2=右键)
    clientX: props.x, // 点击位置的X坐标
    clientY: props.y, // 点击位置的Y坐标
  })

  // 触发事件
  trigger.value?.dispatchEvent(contextMenuEvent)
}

onMounted(() => {
  openContextMenu()
})
</script>

<template>
  <div class="absolute top-100 left-100 bg-white ring-2 ring-red-600">
    target:{{ label }}
  </div>
  <ContextMenu @update:open="handleOpenChange">
    <ContextMenuTrigger>
      <div ref="trigger" />
    </ContextMenuTrigger>
    <ContextMenuContent class="w-64">
      <template v-for="(item, index) in items" :key="index">
        <ContextMenuSeparator v-if="item === 'separator'" />
        <ContextMenuCheckboxItem
          v-else
          :model-value="item.checked"
          @select="item.handler"
        >
          {{ item.label }}
          <ContextMenuShortcut>{{ item.shortcutLabel }}</ContextMenuShortcut>
        </ContextMenuCheckboxItem>
      </template>
    </ContextMenuContent>
  </ContextMenu>
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
