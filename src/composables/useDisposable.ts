import type { Awaitable } from '@vueuse/core'
import { noop } from '@vueuse/core'

type Disposable = () => Awaitable<typeof noop>

export function useDisposable(fn: Disposable) {
  const dispose = ref(noop)

  tryOnMounted(async () => {
    dispose.value = await fn()
  })

  tryOnScopeDispose(() => {
    dispose.value()
  })
}
