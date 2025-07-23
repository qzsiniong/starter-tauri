export function useImgSize(imgSrc: MaybeRefOrGetter<string>) {
  const result = reactive({ width: 0, height: 0, ratio: 1 })

  watchEffect(() => {
    getImgSize(toValue(imgSrc)).then((size) => {
      result.width = size.width
      result.height = size.height
      result.ratio = size.width / size.height
    })
  })

  return readonly(result)
}

function getImgSize(url: string) {
  const promise = new Promise<{ width: number, height: number }>((resolve) => {
    const img = new Image()
    img.onload = function () {
      const width = img.width
      const height = img.height
      resolve({ width, height })
    }
    img.src = url
  })
  return promise
}
