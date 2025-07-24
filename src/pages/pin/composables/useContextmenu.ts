import type { MenuOptions } from '@tauri-apps/api/menu'
import { LogicalPosition } from '@tauri-apps/api/dpi'
import { Menu } from '@tauri-apps/api/menu'
import { cloneDeep } from 'lodash-es'

interface ContextMenuOptions { shadowing: boolean }

interface Handler {
  closeAll: () => void
  close: () => void
  zoom: (ratio: number) => void
  toggleShadow: () => void
}

function getMenus(options: ContextMenuOptions, handler: Handler): NonNullable<MenuOptions['items']> {
  const { shadowing } = options

  const ratios: MenuOptions['items'] = [0.5, 1, 2].map((ratio, i) => ({
    text: `${ratio * 100}%`,
    accelerator: `${i}`,
    action() {
      handler.zoom(ratio)
    },
  }))
  return [
    {
      text: '取消阴影',
      accelerator: 'H',
      checked: !shadowing,
      action() {
        handler.toggleShadow()
      },
    },
    ...ratios,
    { item: 'Separator' },
    {
      text: '关闭',
      // shift+cmd+X 时也会触发
      accelerator: 'cmd+X',
      action() {
        handler.close()
      },
    },
    {
      text: '全部关闭',
      accelerator: 'shift+cmd+X',
      action() {
        handler.closeAll()
      },
    },
  ]
}

export function useContextMenu(handler: Handler, options: MaybeRefOrGetter<ContextMenuOptions>) {
  const magicKeys = useMagicKeys()

  const items = computed(() => {
    return getMenus(toValue(options), handler)
  })

  useEventListener('contextmenu', async (e) => {
    e.preventDefault()

    const menu = await Menu.new({
    // 必须要clone，否则多次调用时，第一次以后action会失效
      items: cloneDeep(items.value),
    })

    menu.popup(new LogicalPosition(e.clientX, e.clientY))
  })

  watchEffect((onCleanup) => {
    const scope = effectScope()

    scope.run(() => {
      for (const item of items.value) {
        const it = item as any
        if (it.item === 'Separator')
          continue

        const { action, accelerator } = it
        watch(magicKeys[accelerator], (v) => {
          v && action()
        })
      }
    })

    onCleanup(() => scope.stop())
  })

  return items
}
