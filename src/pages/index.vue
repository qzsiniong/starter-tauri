<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { Button } from '@/components/ui/button'

async function screenshot() {
  await invoke('take_screenshot')
}

/**
 * 修改快捷键的逻辑
 * @param key - 新的快捷键数组
 */
function changeShortcut() {
  invoke('change_shortcut', { key: 'command+ctrl+option+shift+k' })
}
</script>

<template>
  <div class="p-10">
    <Button @click="screenshot">
      截图
    </Button>
    <Button @click="changeShortcut">
      changeShortcut
    </Button>
    <RouterLink to="/preference/shortcuts">
      <Button>
        /preference/shortcuts
      </Button>
    </RouterLink>
  </div>
</template>

<style scoped>

</style>
