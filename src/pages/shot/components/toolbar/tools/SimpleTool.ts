import type { FabricObject, XY } from 'fabric'
import type { MouseEvent } from './Tool'
import { Tool } from './Tool'

export abstract class SimpleTool extends Tool {
  private drawing = false
  private mouseFrom: XY | null = null

  abstract createShape(e: MouseEvent, mouseFrom: XY): FabricObject[] | FabricObject

  private castArray<T>(value: T | T[]): T[] {
    return Array.isArray(value) ? value : [value]
  }

  // activate(): boolean | void | undefined {
  //   this.canvas.selection = false
  //   this.disableObjectsSelectable()
  // }

  // deactivate() {
  //   this.canvas.selection = true
  //   this.resetObjectsSelectable()
  // }

  onMousedown(e: MouseEvent): void {
    this.mouseFrom = e.viewportPoint
    this.drawing = true
  }

  onMousemove(e: MouseEvent): void {
    if (!this.drawing) {
      return
    }

    super.clearObjects()

    const objects = this.createShape(e, this.mouseFrom!)
    super.addObjects(...this.castArray(objects))
  }

  onMouseup(_e: MouseEvent): void {
    this.drawing = false
    super.saveObjects()

    this.resetObjectsSelectable()
    this.actions.reset()
  }
}
