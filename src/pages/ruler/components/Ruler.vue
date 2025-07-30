<script setup lang="ts">
import type { MenuItemOptions } from '@tauri-apps/api/menu'

import { isTauri } from '@tauri-apps/api/core'
import { LogicalPosition, LogicalSize } from '@tauri-apps/api/dpi'
import { Menu } from '@tauri-apps/api/menu'
import { getAllWebviewWindows, getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { cursorPosition, getCurrentWindow, monitorFromPoint } from '@tauri-apps/api/window'
import { useTauriMousePosition } from '@/composables/useTauriMousePosition'

const props = defineProps<{
  type: 'horizontal' | 'vertical'
}>()

/**
 * 最大长度为 6000px
 */
const MAX_LEN = 6000
const lines = Array.from({ length: MAX_LEN / 2 + 10 }).map((_, index) => {
  const len = 2 * index
  return {
    index,
    len,
    cls: index % 25 === 0 ? 'milestone' : (index % 5 === 0 && 'major'),
    hasLabel: index !== 0 && index % 25 === 0,
    labelCls: `label ${len < 100 ? 'l10' : len < 1000 ? 'l100' : 'l1000'}`,
  }
})

const mousePosition = useTauriMousePosition()

const horizontal = computed(() => {
  return props.type === 'horizontal'
})
const vertical = computed(() => {
  return !unref(horizontal)
})

const mouseOffset = computed(() => {
  return toValue(horizontal) ? mousePosition.x : mousePosition.y
})

if (isTauri()) {
  const currentWebviewWindow = getCurrentWebviewWindow()
  const currentWindow = getCurrentWindow()

  let initialSize = new LogicalSize(0, 0)

  onMounted(async () => {
    const [f, s] = await Promise.all([
      currentWebviewWindow.scaleFactor(),
      currentWebviewWindow.size(),
    ])

    // factor = f
    initialSize = s.toLogical(f)
  })

  async function float(direction: 'left' | 'right' | 'top' | 'bottom') {
    const [size, position, monitor] = await Promise.all([
      currentWindow.outerSize(),
      currentWindow.outerPosition(),
      cursorPosition().then((p) => {
        return monitorFromPoint(p.x, p.y)
      }),
    ])
    if (!monitor) {
      tauriLogWarn(`无法根据鼠标光标位置，获取显示器。`)
      return
    }

    if (direction === 'left') {
      position.x = monitor.position.x
    }
    else if (direction === 'right') {
      position.x = monitor.position.x + monitor.size.width - size.width
    }
    else if (direction === 'top') {
      position.y = monitor.position.y
    }
    else if (direction === 'bottom') {
      position.y = monitor.position.y + monitor.size.height - size.height
    }
    await currentWebviewWindow.setPosition(position)
  }

  async function setLen(len: number) {
    const width = unref(vertical) ? initialSize.width : len
    const height = unref(horizontal) ? initialSize.height : len
    const size = new LogicalSize(width, height)
    await currentWebviewWindow.setSize(size)
  }

  async function closeRulers() {
    const windows = await getAllWebviewWindows()

    const promises = windows.filter(w => w.label.startsWith('ruler_'))
      .map(w => w.close())

    await Promise.all(promises)
  }

  useEventListener('contextmenu', async (e) => {
    e.preventDefault()

    const lens = [100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 2000, 3000, 4000, 6000, 7000]
    const lenMenus: Array<MenuItemOptions> = lens.map(len => ({
      text: `${len}px`,
      action() {
        setLen(len)
      },
    }))

    const menu = await Menu.new({
      items: [
        {
          text: '关闭',
          action() {
            closeRulers()
          },
        },
        {
          text: '←',
          // icon: 'GoLeft',
          action: float.bind(null, 'left'),
        },
        {
          text: '→',
          // icon: 'GoRight',
          action: float.bind(null, 'right'),
        },
        {
          text: '↑',
          action: float.bind(null, 'top'),
        },
        {
          text: '↓',
          action: float.bind(null, 'bottom'),
        },
        ...lenMenus,
      ],
    })

    menu.popup(new LogicalPosition(e.clientX, e.clientY))
  })

  // onMounted(async () => {

  //   currentWebviewWindow.onResized((e) => {
  //     const size = e.payload
  //     console.log('🥖 initialSize', initialSize)
  //     console.log('🥖 size', size)
  //     currentWebviewWindow.setSize(new PhysicalSize(initialSize.width, size.height))
  //   })
  // })
}
</script>

<template>
  <div data-tauri-drag-region class="fixed inset-0 rg-overlay">
    <div
      data-tauri-drag-region
      class="ruler select-none"
      :class="[horizontal ? 'h' : 'v']"
    >
      <div
        data-tauri-drag-region
        class="absolute z-10 bg-red-500"
        :class="[horizontal ? 'top-0 h-full w-[1px]' : 'left-0 w-full h-[1px]']"
        :style="horizontal ? `left:${mouseOffset}px` : `top:${mouseOffset}px`"
      >
        <strong
          v-show="mouseOffset >= 0"
          data-tauri-drag-region
          class="absolute bg-white !text-[10px] text-red-500"
          :class="[
            horizontal && mouseOffset > 50 ? 'right-1' : 'left-1',
            vertical && mouseOffset > 50 ? 'bottom-1' : 'top-1',
          ]"
        >
          {{ mouseOffset }}
        </strong>
      </div>

      <span
        v-for="line in lines" :key="line.index"
        data-tauri-drag-region
        :class="line.cls"
      >
        <span
          v-if="line.hasLabel"
          data-tauri-drag-region
          :class="line.labelCls"
        >
          {{ line.len }}
        </span>
      </span>
    </div>
  </div>
</template>

<style>
html,
body {
  margin: 0;
  padding: 0;
  /* background-color: transparent; */
  background-color: red;
}
</style>

<style scoped>
.rg-overlay * {
  box-sizing: unset;
  white-space: nowrap;
}

.rg-overlay {
  position: absolute;
  top: 0;
  left: 0;
  overflow: hidden;
}

.ruler {
  background-color: #ffffff;
  position: absolute;
  top: 0;
  left: 0;
  z-index: 9990;
}

.ruler .label {
  font: 12px Arial;
  color: #000;
}

.ruler,
.ruler span {
  font-size: 0;
}

.ruler.h {
  /* width: 3000px; */
  left: -1px;
  padding-top: 14px;
  border-bottom: solid 1px #000;
}

.ruler.v {
  /* height: 7000px; */
  top: -1px;
  padding-left: 16px;
  width: 25px;
  border-right: solid 1px #000;
}

.ruler.h span {
  border-left: solid 1px #999;
  height: 9px;
  width: 1px;
  vertical-align: bottom;
  display: inline-block;
  /* *display: inline; */
  zoom: 1;
}

.ruler.v span {
  display: block;
  margin-left: auto;
  margin-right: 0;
  border-top: solid 1px #999;
  width: 9px;
  height: 1px;
}

.ruler.v span.major {
  border-top: solid 1px #000;
  width: 13px;
}

.ruler.v span.milestone {
  position: relative;
  border-top: solid 1px #000;
  width: 17px;
}

.ruler.v span.label {
  border: 0;
  font-size: 9px;
  position: absolute;
  text-align: center;
  width: 9px;
}

.ruler.h span.major {
  border-left: solid 1px #000;
  height: 13px;
}

.ruler.h span.milestone {
  position: relative;
  border-left: solid 1px #000;
  height: 17px;
}

.ruler.h span.label {
  border: 0;
  font-size: 9px;
  position: absolute;
  text-align: center;
  top: -14px;
  width: 9px;
}

.ruler.h .l10 {
  left: -5px;
}

.ruler.h .l100 {
  left: -7px;
}

.ruler.h .l1000 {
  left: -10px;
}

.ruler.v .l10,
.ruler.v .l100,
.ruler.v .l1000 {
  top: -7px;
}

.ruler.v .l10 {
  left: -12px;
}

.ruler.v .l100 {
  left: -17px;
}

.ruler.v .l1000 {
  left: -23px;
}
</style>
