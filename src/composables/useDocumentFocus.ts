import { onMounted, onUnmounted, ref } from 'vue'

export function useDocumentFocus() {
  const isFocused = ref(document.hasFocus())
  const onFocus = () => {
    console.log('focus')
    isFocused.value = true
  }
  const onBlur = () => {
    isFocused.value = false
    console.log('blur')
  }
  onMounted(async () => {
    window.addEventListener('focus', onFocus)
    window.addEventListener('blur', onBlur)
  })

  onUnmounted(() => {
    window.removeEventListener('focus', onFocus)
    window.removeEventListener('blur', onBlur)
  })

  return isFocused
}
