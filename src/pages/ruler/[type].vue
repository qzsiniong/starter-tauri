<script setup lang="ts">
import Ruler from './components/Ruler.vue'

defineProps<{
  type: 'horizontal' | 'vertical'
}>()

definePage({
  props: true,
})
</script>

<template>
  <Ruler :type="type" />
</template>
