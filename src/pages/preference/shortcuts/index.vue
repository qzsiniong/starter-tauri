<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { FormControl, FormField, FormItem, FormLabel } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { useRegister } from '@/composables/useRegister'

useRegister('Command+Shift+Alt+Ctrl+O', (e) => {
  console.log('🍹 e', e)
})
</script>

<template>
  <div>
    <Card class="rounded-sm shadow-none">
      <CardHeader class="border-b">
        <CardTitle>基础设置</CardTitle>
        <!-- <CardDescription>Deploy your new project in one-click.</CardDescription> -->
      </CardHeader>
      <CardContent>
        <FormField v-slot="{ componentField }" name="username">
          <FormItem class="flex">
            <FormLabel>Username</FormLabel>
            <FormControl>
              <Input type="text" placeholder="shadcn" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
      </CardContent>
      <CardFooter class="flex justify-between px-6 pb-6">
        <Button variant="outline">
          Cancel
        </Button>
        <Button>Deploy</Button>
      </CardFooter>
    </Card>
  </div>
</template>

<style scoped>

</style>
