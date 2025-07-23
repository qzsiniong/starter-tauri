import type { XY } from 'fabric'
import type { MouseEvent } from './Tool'
import { Path } from 'fabric'
import { SimpleTool } from './SimpleTool'

export class ArrowTool extends SimpleTool {
  public type: string = 'arrow'
  public title: string = '画箭头'

  private drawWidth = 2
  private color = 'red'

  createShape(e: MouseEvent, mouseFrom: XY) {
    const { x: x0, y: y0 } = mouseFrom!
    const { x: x1, y: y1 } = e.viewportPoint

    const w = x1 - x0
    const h = y1 - y0

    const sh = Math.cos(Math.PI / 4) * 16
    const sin = h / Math.sqrt(w ** 2 + h ** 2)
    const cos = w / Math.sqrt(w ** 2 + h ** 2)
    const w0 = (16 * sin) / 4
    const h0 = (16 * cos) / 4
    const centerX = sh * cos
    const centerY = sh * sin
    /**
     * centerX,centerY 表示起始点，终点连线与箭头尖端等边三角形交点相对x，y
     * w1 ，h1用于确定四个点
     */

    let path = `M ${x0} ${y0}`
    path += ` L ${x1 - centerX + w0} ${y1 - centerY - h0}`
    path += ` L ${x1 - centerX + w0 * 2} ${y1 - centerY - h0 * 2}`
    path += ` L ${x1} ${y1}`
    path += ` L ${x1 - centerX - w0 * 2} ${y1 - centerY + h0 * 2}`
    path += ` L ${x1 - centerX - w0} ${y1 - centerY + h0}`
    path += ' Z'

    return new Path(path, {
      stroke: this.color,
      fill: this.color,
      strokeWidth: this.drawWidth,
    })
  }
}
