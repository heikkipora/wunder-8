const _ = require('lodash')
const Promise = require('bluebird')
const fs = Promise.promisifyAll(require('fs'))

const LINE_MAX_LENGTH = 80
const SPACE_LENGTH = 1

readInput()
  .then(splitToWords)
  .then(groupByLength)
  .then(collectOptimalRows)
  .then(writeOutput)

function splitToWords(text) {
  return text.split(/\s/)
}

function groupByLength(words) {
  return _(words)
    .filter(word => word.length > 0)
    .groupBy('length')
    .value()
}

function collectOptimalRows(groupedWords) {
  const rows = []

  while(Object.keys(groupedWords).length > 0) {
    const words = []
    let lineLength = 0

    while(lineLength < LINE_MAX_LENGTH) {
      const word = longestWord(groupedWords, LINE_MAX_LENGTH - lineLength)
      if (!word) {
        break;
      }
      lineLength += word.length + SPACE_LENGTH
      words.push(word)
    }
    rows.push(words)
  }
  return rows
}

function longestWord(groupedWords, maxLength) {
  const longest = _(groupedWords)
    .keys()
    .filter(length =>length <= maxLength)
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

function readInput() {
  return fs.readFileAsync('./alastalon_salissa.txt', 'utf8')
}

function writeOutput(rows) {
  const text = rows.map(row => row.join(' ')).join('\n')
  return fs.writeFileAsync('./alastalon_salissa_output.txt', text, 'utf8')
}