import type { ShortcutHandler } from '@tauri-apps/plugin-global-shortcut'
import {
  isRegistered,
  register,

  unregister,
} from '@tauri-apps/plugin-global-shortcut'
import { castArray } from 'lodash-es'

export function useRegister(shortcuts: MaybeRefOrGetter<string | string[]>, handler: ShortcutHandler) {
  watchEffect(async () => {
    const cuts = toValue(shortcuts)

    for await (const shortcut of castArray(cuts)) {
      if (!shortcut)
        continue

      const registered = await isRegistered(shortcut)

      if (registered) {
        await unregister(shortcut)
      }
    }

    if (!cuts)
      return

    register(cuts, (event) => {
      if (event.state === 'Released')
        return

      handler(event)
    })
  })

  tryOnMounted(()=>{
    unregister(toValue(shortcuts))
  })
}
