<script setup lang="ts">
import type { SidebarProps } from '@/components/ui/sidebar'
import { on } from 'node:events'
import { SearchIcon, SettingsIcon } from 'lucide-vue-next'
import { Sidebar, SidebarContent, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar'

const props = defineProps<SidebarProps>()

const items = [
  {
    title: '设置',
    url: '/preference/',
    icon: SettingsIcon,
    isActive: false,
  },
  {
    title: '快捷键',
    url: '/preference/shortcuts/',
    icon: SettingsIcon,
    isActive: true,
  },
  {
    title: 'Search',
    url: '#',
    icon: SearchIcon,
    isActive: false,
  },
]

// function dragRegion(ele: Element) {
//   for (const element of ele.children) {
//     element.setAttribute('data-tauri-drag-region', '')
//     dragRegion(element);
//   }
// }

// onMounted(() => {
//   document.querySelectorAll('[data-tauri-drag-region]').forEach((item) => {
//     dragRegion(item)
//   })
// })
</script>

<template>
  <Sidebar class="border-r-0" v-bind="props">
    <div data-tauri-drag-region class="size-full pt-[30px]">
      <SidebarMenu>
        <SidebarMenuItem v-for="item in items" :key="item.title">
          <SidebarMenuButton as-child :is-active="item.isActive">
            <a :href="item.url">
              <component :is="item.icon" />
              <span>{{ item.title }}</span>
            </a>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </div>
  </Sidebar>
</template>

<style scoped>
[data-slot='sidebar-menu-button'] {
  flex-direction: column;
  height: auto;
}
</style>
