const hexAlphaKeyMaps: Record<string, number> = {}
const hexAlphaNumMaps: Record<string, string> = {}

'FF,FC,FA,F7,F5,F2,F0,ED,EB,E8,E6,E3,E0,DE,DB,D9,D6,D4,D1,CF,CC,C9,C7,C4,C2,BF,BD,BA,B8,B5,B3,B0,AD,AB,A8,A6,A3,A1,9E,9C,99,96,94,91,8F,8C,8A,87,85,82,80,7D,7A,78,75,73,70,6E,6B,69,66,63,61,5E,5C,59,57,54,52,4F,4D,4A,47,45,42,40,3D,3B,38,36,33,30,2E,2B,29,26,24,21,1F,1C,1A,17,14,12,0F,0D,0A,08,05,03,00'.split(',').forEach((key, i) => {
  const num = (100 - i) / 100
  hexAlphaKeyMaps[key] = num
  hexAlphaNumMaps[num] = key
})

export function rgbToHex(rgbRest: {
  r: number
  g: number
  b: number
  a?: number
}) {
  if (!rgbRest) {
    return ''
  }
  const { r, g, b, a = 1 } = rgbRest
  const hexR = r.toString(16).padStart(2, '0')
  const hexG = g.toString(16).padStart(2, '0')
  const hexB = b.toString(16).padStart(2, '0')
  const hexA = a === 1 ? '' : (hexAlphaNumMaps[a] || '')

  // 返回十六进制颜色代码
  return `#${hexR}${hexG}${hexB}${hexA}`.toUpperCase()
}
