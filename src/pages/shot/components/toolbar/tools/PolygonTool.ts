import type { XY } from 'fabric'
import type { MouseEvent } from './Tool'
import { Circle, Polygon as FabricPolygon, Line } from 'fabric'
import { Tool } from './Tool'

export class PolygonTool extends Tool {
  public type: string = 'polygon'
  public title: string = '画多边形'

  private drawWidth = 2
  private color = 'red'

  private drawing = false
  private activeShape: FabricPolygon | null = null // 当前绘制对象
  private points: Circle[] = []

  private lineArray: Line[] = []
  private activeLine: Line | null = null
  private line: Line | null = null

  private isClose(e: MouseEvent) {
    if (!e.target || this.points.length <= 1)
      return

    // e.target.id == this.points[0].id 表示点击了初始红点
    return (e.target as any).id === (this.points[0] as any).id
  }

  private addPoint(e: MouseEvent) {
    const random = Math.floor(Math.random() * 10000)
    const id = new Date().getTime() + random
    const isStart = this.points.length === 0
    const circle = new Circle({
      radius: 5,
      fill: isStart ? 'red' : '#ffffff',
      stroke: '#333333',
      strokeWidth: 0.5,
      left: (e.viewportPoint.x) / this.canvas.getZoom(),
      top: (e.viewportPoint.y) / this.canvas.getZoom(),
      selectable: false,
      hasBorders: false,
      hasControls: false,
      originX: 'center',
      originY: 'center',
      id,
      objectCaching: false,
    })
    const points = [
      (e.viewportPoint.x) / this.canvas.getZoom(),
      (e.viewportPoint.y) / this.canvas.getZoom(),
      (e.viewportPoint.x) / this.canvas.getZoom(),
      (e.viewportPoint.y) / this.canvas.getZoom(),
    ] as any

    this.line = new Line(points, {
      strokeWidth: 2,
      fill: '#999999',
      stroke: '#999999',
      class: 'line',
      originX: 'center',
      originY: 'center',
      selectable: false,
      hasBorders: false,
      hasControls: false,
      evented: false,

      objectCaching: false,
    })
    if (this.activeShape) {
      const pos = this.canvas.getViewportPoint(e.e)
      const points = this.activeShape.get('points')
      points.push({
        x: pos.x,
        y: pos.y,
      })
      const polygon = new FabricPolygon(points, {
        stroke: '#333333',
        strokeWidth: 1,
        fill: '#cccccc',
        opacity: 0.3,

        selectable: false,
        hasBorders: false,
        hasControls: false,
        evented: false,
        objectCaching: false,
      })
      this.canvas.remove(this.activeShape)
      this.canvas.add(polygon)
      this.activeShape = polygon
      this.canvas.renderAll()
    }
    else {
      const polyPoint = [
        {
          x: (e.viewportPoint.x) / this.canvas.getZoom(),
          y: (e.viewportPoint.y) / this.canvas.getZoom(),
        },
      ]
      const polygon = new FabricPolygon(polyPoint, {
        stroke: '#333333',
        strokeWidth: 1,
        fill: '#cccccc',
        opacity: 0.3,

        selectable: false,
        hasBorders: false,
        hasControls: false,
        evented: false,
        objectCaching: false,
      })
      this.activeShape = polygon
      this.canvas.add(polygon)
    }
    this.activeLine = this.line

    this.points.push(circle)
    this.lineArray.push(this.line!)
    this.canvas.add(this.line)
    this.canvas.add(circle)
  }

  private generatePolygon() {
    const points: XY[] = []
    this.points.forEach((point) => {
      points.push({
        x: point.left,
        y: point.top,
      })
      this.canvas.remove(point)
    })
    this.lineArray.forEach((line) => {
      this.canvas.remove(line)
    })
    this.canvas.remove(this.activeShape!)
    this.canvas.remove(this.activeLine!)
    const polygon = new FabricPolygon(points, {
      stroke: this.color,
      strokeWidth: this.drawWidth,
      fill: 'rgba(255, 255, 255, 0)',
      opacity: 1,
      hasBorders: true,
      hasControls: false,
    })
    this.canvas.add(polygon)

    this.points = [] // 顶点集合
    this.lineArray = [] // 线集合
    this.activeLine = null
    this.activeShape = null
    this.drawing = false

    this.resetObjectsSelectable()
    this.actions.reset()
  }

  onMousedown(e: MouseEvent): void {
    this.drawing = true
    if (this.isClose(e)) {
      this.generatePolygon()
    }
    else {
      this.addPoint(e)
    }
  }

  onMousemove(e: MouseEvent): void {
    if (!this.drawing) {
      return
    }

    if (this.activeLine) {
      const pointer = e.viewportPoint
      this.activeLine.set({ x2: pointer.x, y2: pointer.y })

      const points = this.activeShape!.get('points')
      points[this.points.length] = {
        x: pointer.x,
        y: pointer.y,
        zIndex: 1,
      }
      this.activeShape!.set({
        points,
      })
      this.canvas.renderAll()
    }
    this.canvas.renderAll()
  }
}
