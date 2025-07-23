import type { XY } from 'fabric'
import type { MouseEvent } from './Tool'
import { Ellipse as FabricEllipse } from 'fabric'
import { SimpleTool } from './SimpleTool'

export class EllipseTool extends SimpleTool {
  public type: string = 'ellipse'
  public title: string = '画圆'

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

    return new FabricEllipse({
      left: width / 2 + mouseFrom.x,
      top: height / 2 + mouseFrom.y,
      stroke: this.color,
      fill: 'rgba(255, 255, 255, 0)',
      originX: 'center',
      originY: 'center',
      rx: Math.abs(width) / 2,
      ry: Math.abs(height) / 2,
      strokeWidth: this.drawWidth,
    })
  }
}
