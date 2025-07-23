interface MenuItem {
  label: string
  shortcut: string
  shortcutLabel: string
  checked?: boolean
  handler: () => void
}

export interface ContextMenuOptions { shadowing: boolean }

export type ContextMenuItemEvent = {
  type: 'ratio'
  payload: number
} | {
  type: 'close'
} | {
  type: 'closeAll'
}
| {
  type: 'toggleShadow'
}

export type EventHandler = (event: ContextMenuItemEvent) => void

function getMenus(options: ContextMenuOptions, handler: EventHandler): Array<MenuItem | 'separator'> {
  const { shadowing } = options

  const ratios: MenuItem[] = [0.5, 1, 2].map((ratio, i) => ({
    label: `${ratio * 100}%`,
    shortcut: `${i}`,
    shortcutLabel: `${i}`,
    handler() {
      handler({ type: 'ratio', payload: ratio })
    },
  }))
  return [
    {
      label: '取消阴影',
      shortcut: 'H',
      shortcutLabel: 'H',
      checked: !shadowing,
      handler() {
        handler({ type: 'toggleShadow' })
      },
    },
    ...ratios,
    'separator',
    {
      label: '关闭',
      shortcut: 'meta+W',
      shortcutLabel: '⌘W',
      handler() {
        handler({ type: 'close' })
      },
    },
    {
      label: '全部关闭',
      shortcut: 'meta+shift+W',
      shortcutLabel: '⇧⌘W',
      handler() {
        handler({ type: 'closeAll' })
      },
    },
  ]
}

export function useContextMenuItems(handler: EventHandler, options: MaybeRefOrGetter<ContextMenuOptions>) {
  const magicKeys = useMagicKeys()

  const items = computed(() => {
    return getMenus(toValue(options), handler)
  })

  watchEffect((onCleanup) => {
    const scope = effectScope()

    scope.run(() => {
      for (const item of items.value) {
        if (item === 'separator')
          continue

        const { handler, shortcut } = item
        watch(magicKeys[shortcut], (v) => {
          v && handler()
        })
      }
    })

    onCleanup(() => scope.stop())
  })

  return items
}
