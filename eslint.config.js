import antfu from '@antfu/eslint-config'

export default antfu({
  formatters: true,
  vue: true,

  typescript: {
    overrides: {
      'no-console': 'off',
    },
  },

  ignores: ['src-tauri/target/**'],
})
