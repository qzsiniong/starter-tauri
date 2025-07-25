export interface Size {
  width: number
  height: number
}

/**
 * 限制宽高尺寸，保持原始宽高比
 * @param originalSize 原始尺寸
 * @param maxSize 最大限制尺寸
 * @param minSize 最小限制尺寸（可选）
 * @returns 调整后的尺寸，保持原始宽高比且在最大和最小限制范围内
 */
export function sizeConstraints(originalSize: Size, maxSize: Size, minSize: Size = { width: 0, height: 0 }): Size {
  // 计算最大限制比例
  const maxWidthRatio = maxSize.width / originalSize.width
  const maxHeightRatio = maxSize.height / originalSize.height
  const maxRatio = Math.min(maxWidthRatio, maxHeightRatio, 1)

  // 应用最大限制比例
  const constrainedSize = {
    width: Math.round(originalSize.width * maxRatio),
    height: Math.round(originalSize.height * maxRatio),
  }

  // 计算最小限制比例
  const minWidthRatio = minSize.width / constrainedSize.width
  const minHeightRatio = minSize.height / constrainedSize.height
  const minRatio = Math.max(minWidthRatio, minHeightRatio, 1)

  // 应用最小限制比例
  return {
    width: Math.round(constrainedSize.width * minRatio),
    height: Math.round(constrainedSize.height * minRatio),
  }
}
