import type { Plugin } from 'vite'
import path from 'node:path'
import process from 'node:process'
// 导入 magic-string 包，它可以让你修改字符串并生成 sourcemap
import MagicString from 'magic-string'
// 导入 rollup-pluginutils 包，它提供了一些工具函数来处理 sourcemap
import { createFilter } from 'rollup-pluginutils'

interface Options {
  handles?: Array<{ src: string, target?: string }>
  cwd?: string
  include?: Array<string | RegExp> | string | RegExp | null
  exclude?: Array<string | RegExp> | string | RegExp | null
}

/**
 *文件内容替换: 两个关键字fileName,row
 * @param options {handles:
 * [{src: '', 待替换的字符串
 * target: ''}], 目标字符串
 * include: '', 包含的文件
 * exclude: ''} 排除的文件
 *
 * 示例配置： replace({
            handles: [{src: 'log-position-placeholder'}, {src: 'console\\.log\\(', target: 'console.log("调试输出:  fileName:row ==> " +'}], // 设置占位符数组
        })
 将log-position-placeholder替换成文件名加行号，以及将console.log( 替换成 console.log("调试输出: 文件名加行号
 注意src里面的特殊符合需要用两个\\来进行转义
 * @return
 */
export default function replace(options: Options = {}): Plugin {
  options = Object.assign({ handles: [{ src: '', target: '' }] }, options)
  const { cwd = process.cwd() } = options
  let fileCount = 0
  let replaceCount = 0
  // 使用 createFilter 函数来过滤掉不需要处理的文件
  const filter = createFilter(options.include, options.exclude)
  return {
    // 插件名
    name: 'placeholder-replace',

    transform: {
      order: 'pre',
      handler(code, id) {
        // 如果文件不符合过滤条件，直接返回
        if (!filter(id))
          return null
        // 将代码转换为 UTF-8 编码
        // code = code.toString('utf-8')

        // 创建一个 MagicString 对象，用于修改代码和生成 sourcemap
        const magicString = new MagicString(code)

        // 获取文件名
        // const fileName = path.basename(id)
        const fileName = path.relative(cwd, id)
        // 遍历每个占位符
        options.handles.forEach((handle) => {
        // 构建占位符的正则表达式
          const placeholderRegex = new RegExp(`${handle.src}`, 'g')
          let result: RegExpExecArray | null = null
          // eslint-disable-next-line no-cond-assign
          while ((result = placeholderRegex.exec(code))) {
            const start = result.index
            const end = start + result[0].length

            // 获取行号和列号
            const line = getRow(result.input, start, '\n')

            // 替换占位符为文件名:行号=>>的形式
            let target = handle.target

            if (target) {
              target = target.includes('[row]') ? target.replace('[row]', `${line}`) : target
              target = target.includes('[fileName]') ? target.replace('[fileName]', fileName) : target
            }
            else {
              target = `${fileName}:${line}`
            }
            magicString.overwrite(start, end, target)

            replaceCount++
          }
        })

        fileCount++

        // 返回修改后的代码和 sourcemap 对象
        return {
          code: magicString.toString(),
          map: magicString.generateMap({ hires: true }),
        }
      },
    },

    buildEnd() {
      console.log(`\n替换的文件数: ${fileCount}`)
      console.log(`\n替换的次数: ${replaceCount}`)
    },
  }
}

/**
 * 获取行号
 * @param str 代码内容
 * @param end  截止位置
 * @param tag  换行符
 * @return {number} 行号
 */
function getRow(str: string, end: number, tag: string) {
  const slice = str.slice(0, end)
  const split = slice.split(tag)
  const length = split.length
  return length
}
