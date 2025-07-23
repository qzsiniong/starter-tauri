<script setup lang="ts">
import type { Canvas } from 'fabric'

import { PinIcon } from 'lucide-vue-next'
import { Toggle } from '@/components/ui/toggle'
import { TooltipProvider } from '@/components/ui/tooltip'
import { useTools } from './useTools'

const props = defineProps({
  canvas: { type: Object as PropType<Canvas>, required: true },
})

const keys = useMagicKeys()
const { tools, activatedTool } = useTools(props.canvas)

tools.forEach((t) => {
  watch(keys[t.shortcut], (v) => {
    if (v)
      t.active()
  })
})

function handlePin() {

}
</script>

<template>
  <div class="border bg-white/80 rounded-lg p-1 flex gap-1 shadow-md">
    <TooltipProvider>
      <!-- <ToggleGroup type="single">
        <ToggleGroupItem v-for="tool in tools" :key="tool.type" :value="tool.type">
          <component :is="tool.icon" />
        </ToggleGroupItem>
      </ToggleGroup> -->

      <Toggle
        v-for="tool in tools" :key="tool.type"
        class="relative"
        :model-value="activatedTool?.type === tool.type"
        @click="tool.active"
      >
        <component
          :is="tool.icon"
          class="repeat-1"
          :class="{ 'animate-bounce': activatedTool?.type === tool.type }"
        />
        <span class="text-[10px] absolute bottom-1 right-1">{{ tool.shortcut }}</span>
      </Toggle>
      <Toggle
        class="relative"
        @click="handlePin"
      >
        <PinIcon />
        <span class="text-[10px] absolute bottom-1 right-1">p</span>
      </Toggle>
    </TooltipProvider>
  </div>
</template>

<style scoped>

</style>
