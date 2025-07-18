import type { XY } from 'fabric'
import type { MouseEvent } from './Tool'
import { Rect } from 'fabric'
import { SimpleTool } from './SimpleTool'

export class RectangleTool extends SimpleTool {
  public type: string = 'rectangle'
  public title: string = '画矩形'

  private drawWidth = 2
  private color = 'red'

  createShape(e: MouseEvent, mouseFrom: XY) {
    const mouseTo = e.viewportPoint

    let width = mouseTo.x - mouseFrom.x
    const height = mouseTo.y - mouseFrom.y

    // 按shift时画正方型
    if (e.e.shiftKey) {
      width = Math.abs(height) * (width > 0 ? 1 : -1)
    }

    return new Rect({
      left: mouseFrom.x,
      top: mouseFrom.y,
      fill: null,
      stroke: this.color,
      strokeWidth: this.drawWidth,
      width,
      height,
    })
  }
}
