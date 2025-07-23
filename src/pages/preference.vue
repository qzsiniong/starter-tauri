<script setup lang="ts">
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar'
import Sidebar from './preference/components/Sidebar.vue'
</script>

<template>
  <SidebarProvider
    :style="{
      '--sidebar-width': '100px',
    }"
  >
    <Sidebar />
    <SidebarInset class="p-4">
      <div data-tauri-drag-region class="h-4 fixed top-0 left-0 right-0 " />
      <RouterView />
    </SidebarInset>
  </SidebarProvider>
</template>

<style scoped>

</style>
