<script setup lang="ts">
import type { Canvas } from 'fabric'
import FabricCanvas from './components/FabricCanvas.vue'
import Toolbar from './components/toolbar/Toolbar.vue'

const canvas = ref<Canvas>()
</script>

<template>
  <div>
    <FabricCanvas @ready="canvas = $event" />

    <Toolbar v-if="canvas" :canvas="canvas" class="fixed bottom-10 left-10" />
  </div>
</template>

<style scoped>

</style>
