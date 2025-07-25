import type { LogicalSize } from '@tauri-apps/api/dpi'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'

export function useWebviewWindowInitialSize() {
  const currentWindow = getCurrentWebviewWindow()

  const initialSize = ref<LogicalSize>()

  onMounted(async () => {
    const [scaleFactor, size] = await Promise.all([
      currentWindow.scaleFactor(),
      currentWindow.size(),
    ])
    initialSize.value = size.toLogical(scaleFactor)
  })

  return readonly(initialSize)
}
