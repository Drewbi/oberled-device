let segmentMap = [0, 8, 40, 32, 64, 72, 104, 96, 128, 136, 168, 160, 192, 200, 232, 224]
let segmentSequence = [ 1, 4, -1, 4 ]
let list = []
let segmentStart = 0
while(list.length < 256){
  segmentSequence.forEach(segmentJump => {
    let index = segmentStart
    for(let i = 0; i < 8; i++){
      list.push(index)
      index++
    }
    index += 15
    for(let i = 8; i > 0; i--){
      list.push(index)
      index--
    }
    segmentStart += segmentJump * 8
  })
}

// console.log(list)
// const matrixMap = new Array(16);
// for(let i = 0; i < matrixMap.length; i++) {
//   matrixMap[i] = []
// }
// list.forEach((item, i) => matrixMap[i % 16].push(item))

// const rotatedList = matrixMap[0].map((_, colIndex) => matrixMap.map(row => row[colIndex]));
// rotatedList.forEach((list) => list.reverse())

// const result = rotatedList.reduce((acc, curr) => [...acc, ...curr])

function decimalToHex(number) {
  return "0x" + number.toString(16).padStart(2, "0");
}

const ogList = [...Array(256).keys()]

// const diffLists = list.reduce((acc, curr, i) => {
//   return `${acc}, ${ogList[i] - curr}`
// }, '')

const formatted = ogList.reduce((acc, curr, i) => {
  return `${acc}, ${decimalToHex(curr)}`
}, '')

console.log(formatted)
