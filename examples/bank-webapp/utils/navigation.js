export const cookiesStrToObj = str => {
  str = str.split(' ')
  const result = {}
  for (let i = 0; i < str.length; i++) {
    const cur = str[i].split('=')
    result[cur[0]] = cur[1]
  }
  return result
}
