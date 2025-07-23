import type { Canvas } from 'fabric'
import type { Actions, Tool } from './tools/Tool'

import { BrushCleaningIcon, CircleIcon, MousePointer2Icon, MoveUpRightIcon, PenIcon, PentagonIcon, SquareIcon, TypeIcon } from 'lucide-vue-next'
import { ArrowTool } from './tools/ArrowTool'
import { ClearTool } from './tools/ClearTool'
import { EllipseTool } from './tools/EllipseTool'
import { PenTool } from './tools/PenTool'
import { PolygonTool } from './tools/PolygonTool'
import { RectangleTool } from './tools/RectangleTool'
import { SelectTool } from './tools/SelectTool'
import { TextTool } from './tools/TextTool'

export function useTools(canvas: Canvas) {
  const activatedTool = ref<Tool>()
  let selectTool: Tool
  const actions: Actions = {
    reset() {
      console.log('🍲 reset')
      activatedTool.value?.deactivate()

      activatedTool.value = selectTool
    },
  }
  selectTool = new SelectTool(canvas, actions)
  activatedTool.value = selectTool

  function activeTool(tool: Tool) {
    activatedTool.value?.deactivate()

    const activated = tool.activate() ?? true

    activatedTool.value = tool
    if (!activated) {
      setTimeout(() => {
        activatedTool.value = selectTool
      }, 300)
    }
  }

  useEventListener(window, 'keyup', (e) => {
    // console.log('🥪 e', e)
    if (e.key === 'Escape' && activatedTool.value) {
      activatedTool.value.deactivate()
      activatedTool.value = undefined
    }
    if (e.key === 'Backspace') {
      canvas.getActiveObjects().forEach((item) => {
        canvas.remove(item)
      })
    }
  })

  canvas.on('mouse:down', (e) => {
    unref(activatedTool)?.onMousedown(e)
  })
  canvas.on('mouse:move', (e) => {
    unref(activatedTool)?.onMousemove(e)
  })
  canvas.on('mouse:up', (e) => {
    unref(activatedTool)?.onMouseup(e)
  })

  const tools = [
    { type: 'select', icon: MousePointer2Icon, title: '自由选择', tool: selectTool },
    { type: 'pen', icon: PenIcon, title: '画笔', tool: new PenTool(canvas, actions) },
    { type: 'arrow', icon: MoveUpRightIcon, title: '画箭头', tool: new ArrowTool(canvas, actions) },
    { type: 'text', icon: TypeIcon, title: '文本输入', tool: new TextTool(canvas, actions) },
    { type: 'rectangle', icon: SquareIcon, title: '画矩形', tool: new RectangleTool(canvas, actions) },
    { type: 'ellipse', icon: CircleIcon, title: '画圆', tool: new EllipseTool(canvas, actions) },
    { type: 'polygon', icon: PentagonIcon, title: '画多边形', tool: new PolygonTool(canvas, actions) },
    { type: 'clear', icon: BrushCleaningIcon, title: '清除', tool: new ClearTool(canvas, actions) },
  ].map((item, i) => ({
    ...item,
    shortcut: item.type === 'clear' ? '0' : `${i + 1}`,
    active: () => activeTool(item.tool),
  }))

  return { tools, activatedTool }
}
