import { Tool } from './Tool'

export class ClearTool extends Tool {
  public type: string = 'clear'
  public title: string = '清空'

  activate() {
    this.canvas.clear()

    return false
  }
}
