import { PencilBrush } from 'fabric'
import { Tool } from './Tool'

export class PenTool extends Tool {
  public type: string = 'pen'
  public title: string = '画笔'

  activate() {
    super.activate()
    this.canvas.isDrawingMode = true
    this.canvas.freeDrawingBrush = new PencilBrush(this.canvas)
  }

  deactivate(): void {
    super.deactivate()
    this.canvas.isDrawingMode = false
    this.canvas.freeDrawingBrush = undefined
  }
}
