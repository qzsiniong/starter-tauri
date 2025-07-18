import type { Canvas, FabricObject, TPointerEvent, TPointerEventInfo } from 'fabric'
import { noop } from '@vueuse/core'

export type MouseEvent = TPointerEventInfo<TPointerEvent> & {
  // alreadySelected: boolean
}

export interface Actions {
  reset: () => void
}

export abstract class Tool {
  public type: string = ''
  public title: string = ''

  private objects: FabricObject[] = []

  constructor(protected canvas: Canvas, protected actions: Actions) { }

  /**
   * 停用所有object可选
   */
  protected disableObjectsSelectable() {
    this.canvas.getObjects().forEach((object) => {
      object.__selectable = object.selectable
      object.selectable = false
    })
  }

  /**
   * 停用所有object可选
   */
  protected resetObjectsSelectable() {
    this.canvas.getObjects().forEach((object) => {
      object.selectable = object.__selectable ?? object.selectable
    })
  }

  protected saveObjects() {
    this.objects = []
  }

  protected clearObjects() {
    this.canvas.remove(...this.objects)
  }

  protected addObjects(...objects: FabricObject[]) {
    this.objects.push(...objects)
    this.canvas.add(...objects)
  }

  /**
   * 激活时触发
   */
  activate(): boolean | void | undefined {
    this.canvas.selection = false
    this.disableObjectsSelectable()
  }

  /**
   * 退出时触发
   */
  deactivate(): void {
    this.canvas.selection = true
    this.resetObjectsSelectable()
  }

  /**
   * 鼠标按下时触发
   */
  onMousedown(_e: MouseEvent): void { }

  /**
   * 鼠标移动时触发
   */
  onMousemove(_e: MouseEvent): void { }

  /**
   * 鼠标松开时触发
   */
  onMouseup(_e: MouseEvent): void { }
}
