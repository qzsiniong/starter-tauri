import { Canvas } from 'fabric'

interface Options {
}

export function useFabric(el: MaybeRef<null | string | HTMLCanvasElement>, _options: Options) {
  const { width, height } = useWindowSize()
  const canvas = shallowRef<Canvas>()

  const size = computed(() => {
    return {
      width: unref(width),
      height: unref(height),
    }
  })

  watch(() => toValue(el), (e) => {
    if (!e)
      return

    canvas.value = new Canvas(e, {
      ...unref(size),
    })

    console.log('🍥 canvas.value', canvas.value)
  })

  watchPostEffect(() => {
    console.log('🍛 unref(size)', unref(size))
    unref(canvas)?.setDimensions(unref(size))
  })

  // const clear = () => {
  //   console.log('🥩 clear')
  //   unref(canvas)?.clear()
  // }

  return canvas
}
