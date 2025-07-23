import type { EventCallback, EventName } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { noop } from '@vueuse/core'

export function useTauriListen<T>(...args: Parameters<typeof listen<T>>) {
  const unlisten = ref(noop)

  tryOnMounted(async () => {
    unlisten.value = await listen<T>(...args)
  })

  tryOnScopeDispose(() => {
    unlisten.value()
  })
}

export function useTauriListenCurrent<T>(event: EventName, handler: EventCallback<T>) {
  const { label } = getCurrentWebviewWindow()

  return useTauriListen(event, handler, {
    target: {
      kind: 'AnyLabel',
      label,
    },
  })
}

// export function useTauriListenCurrent<T>(event: EventName, handler: EventCallback<T>) {
//   const { listen } = getCurrentWebviewWindow()

//   useDisposable(() => listen<T>(event, handler))
// }
