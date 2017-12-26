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

function collectOptimalRows(wordsByLength) {
  const rows = []
  while(hasWordsLeft(wordsByLength)) {
    rows.push(collectOptimalRow(wordsByLength))
  }
  return rows.join('\n')
}

function hasWordsLeft(wordsByLength) {
  return Object.keys(wordsByLength).length > 0
}

function collectOptimalRow(wordsByLength) {
  let lineLength = 0
  const words = []

  while(lineLength < LINE_MAX_LENGTH) {
    const word = longestWord(wordsByLength, LINE_MAX_LENGTH - lineLength)
    if (!word) {
      break;
    }
    lineLength += word.length + SPACE_LENGTH
    words.push(word)
  }
  return words.join(' ')
}

function longestWord(wordsByLength, maxLength) {
  return _(wordsByLength)
    .keys()
    .filter(length => length <= maxLength)
    .takeRight(1)
    .map(longestLength => takeWord(wordsByLength, longestLength))
    .last()
}

function takeWord(wordsByLength, length) {
  const word = wordsByLength[length].pop()
  if (wordsByLength[length].length === 0) {
    delete wordsByLength[length]
  }
  return word
}

function readInput() {
  return fs.readFileAsync('./alastalon_salissa.txt', 'utf8')
}

function writeOutput(text) {
  return fs.writeFileAsync('./alastalon_salissa_output.txt', text, 'utf8')
}