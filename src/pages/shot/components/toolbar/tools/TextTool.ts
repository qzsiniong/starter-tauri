import type { MouseEvent } from './Tool'
import { Textbox } from 'fabric'
import { Tool } from './Tool'

export class TextTool extends Tool {
  public type: string = 'text'
  public title: string = '文本输入'

  private textBox: Textbox | null = null

  private color = 'red'

  /**
   * 激活时触发
   */
  activate(): boolean | void | undefined {
    return super.activate()
  }

  /**
   * 退出时触发
   */
  deactivate(): void {
    super.deactivate()

    // if (this.textBox) {
    //   this.textBox.exitEditing()
    //   this.textBox = null
    // }
  }

  onMousedown(e: MouseEvent): void {
    if (this.textBox && e.target !== this.textBox) {
      this.textBox.exitEditing()
      this.textBox = null
    }
    const { x: left, y: top } = e.viewportPoint
    const textbox = new Textbox('', {
      left,
      top: top - 20,
      fontSize: 16,
      borderColor: this.color,
      fill: this.color,
    })
    this.textBox = textbox
    this.canvas.add(textbox)

    setTimeout(() => {
      textbox.enterEditing()

      this.actions.reset()
    }, 20)
  }
}
