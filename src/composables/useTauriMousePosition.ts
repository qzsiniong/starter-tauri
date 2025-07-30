import { isTauri } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { events } from '@/bindings'
import { useDisposable } from './useDisposable'

export function useTauriMousePosition() {
  const mousePosition = reactive({
    x: 0,
    y: 0,
  })

  if (isTauri()) {
    useDisposable(() => {
      return events.mouseMoveEvent.listen(async (e) => {
        const currentWindow = getCurrentWindow()

        const [scale, position] = await Promise.all([
          currentWindow.scaleFactor(),
          currentWindow.outerPosition(),
        ])

        const pos = position.toLogical(scale)

        mousePosition.x = Math.round(e.payload.x - pos.x)
        mousePosition.y = Math.round(e.payload.y - pos.y)
      })
    })
  }
  else {
    useEventListener('mousemove', (e) => {
      mousePosition.x = e.x
      mousePosition.y = e.y
    })
  }

  return mousePosition
}
