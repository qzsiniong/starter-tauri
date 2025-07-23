import { invoke } from '@tauri-apps/api/core'

export function useContextmenu(options: MaybeRefOrGetter) {
  useEventListener('contextmenu', (e) => {
    e.preventDefault()

    invoke('open_pin_context_menu', {
      options: JSON.stringify(toValue(options)),
    })
  })
}
