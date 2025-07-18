<script setup lang="ts">
import type { Canvas } from 'fabric'

import { Toggle } from '@/components/ui/toggle'
import { TooltipProvider } from '@/components/ui/tooltip'
import { useTools } from './useTools'

const props = defineProps({
  canvas: { type: Object as PropType<Canvas>, required: true },
})

const { tools, activatedTool } = useTools(props.canvas)
</script>

<template>
  <div>
    <TooltipProvider>
      <!-- <ToggleGroup type="single">
        <ToggleGroupItem v-for="tool in tools" :key="tool.type" :value="tool.type">
          <component :is="tool.icon" />
        </ToggleGroupItem>
      </ToggleGroup> -->

      <Toggle
        v-for="tool in tools" :key="tool.type"
        :model-value="activatedTool?.type === tool.type"
        @click="tool.active"
      >
        <component :is="tool.icon" />
      </Toggle>
    </TooltipProvider>
  </div>
</template>

<style scoped>

</style>
