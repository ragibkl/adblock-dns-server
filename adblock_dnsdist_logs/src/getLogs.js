const fs = require('fs')
const pify = require('pify')
const yaml = require('yaml')

const { logFile } = require('./config')

const readFileAsync = pify(fs.readFile)

function getIp (ctx) {
  const requestIp = ctx.ip
  if (requestIp.startsWith('::ffff:')) {
    return requestIp.replace('::ffff:', '')
  }
  return requestIp
}

function extract (message) {
  const split = message.split('\n')

  const qIndex = split.findIndex(s => s === ';; QUESTION SECTION:') + 1
  const question = split[qIndex].replaceAll('\t', ' ')

  const aStart = split.findIndex(s => s === ';; ANSWER SECTION:') + 1
  const fromAnswers = split.slice(aStart)

  const asLimit = fromAnswers.findIndex(s => s === '')
  const answers = fromAnswers.slice(0, asLimit).map(s => s.replaceAll('\t', ' '))

  return [question, answers]
}

async function getLogs (ctx) {
  const ip = getIp(ctx)
  const text = await readFileAsync(logFile, 'utf8')
  const data = yaml.parseAllDocuments(text).map(a => a.toJSON())

  const queries = data.filter(Boolean).map(d => {
    const {
      query_address: queryAddress,
      response_address: responseAddress,
      response_message: responseMessage
    } = d.message
    const [question, answers] = extract(responseMessage)

    return {
      queryAddress,
      responseAddress,
      question,
      answers
    }
  })

  await ctx.render('views/getLogs', { ip, queries })
}

module.exports = {
  getLogs
}
