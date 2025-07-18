<script setup lang="ts">
import { Canvas } from 'fabric'

const emit = defineEmits<{
  ready: [Canvas]
}>()

const el = useTemplateRef('el')

const { width, height } = useWindowSize()
const canvas = shallowRef<Canvas>()

const size = computed(() => {
  return {
    width: unref(width),
    height: unref(height),
  }
})

watchPostEffect(() => {
  console.log('🍛 unref(size)', unref(size))
  unref(canvas)?.setDimensions(unref(size))
})

onMounted(() => {
  canvas.value = new Canvas(el.value!, {
    ...unref(size),
  })

  emit('ready', canvas.value)
})
</script>

<template>
  <canvas ref="el" />
</template>

<style scoped>

</style>
