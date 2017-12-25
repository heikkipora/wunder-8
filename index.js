const _ = require('lodash')
const Promise = require('bluebird')
const fs = Promise.promisifyAll(require('fs'))

readInput()
  .then(splitToWords)
  .then(groupByLength)
  .then(renderOptimalRows)
  .then(logPagesCount)
  .then(writeOutput)

function readInput() {
  return fs.readFileAsync('./alastalon_salissa.txt', 'utf8')
}

function splitToWords(text) {
  return text.split(/\s/)
}

function groupByLength(words) {
  return _(words)
    .filter(word => word.length > 0)
    .groupBy('length')
    .value()
}

function renderOptimalRows(groupedWords) {
  const rows = []
  const lengths = Object.keys(groupedWords).map(Number)

  while(Object.keys(groupedWords).length > 0) {
    const words = []
    let totalLen = 0

    while(totalLen < 80) {
      const word = longestWord(groupedWords, 80 - totalLen)
      if (!word) {
        break;
      }
      totalLen += word.length + 1
      words.push(word)
    }

    rows.push(words)
  }
  return rows
}

function longestWord(groupedWords, maxLen) {
  const longest = _(groupedWords)
    .keys()
    .filter(len => len <= maxLen)
    .last()
  if (!longest) {
    return
  }
  const word = groupedWords[longest].pop()
  if (groupedWords[longest].length === 0) {
    delete groupedWords[longest]
  }
  return word
}

function logPagesCount(rows) {
  console.log(`Result: ${rows.length} rows = ${rows.length / 25} pages`)
  return rows
}

function writeOutput(rows) {
  const text = rows.map(row => row.join(' ')).join('\n')
  return fs.writeFileAsync('./alastalon_salissa_output.txt', text, 'utf8')
}