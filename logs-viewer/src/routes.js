const fs = require('fs')
const Router = require('@koa/router')
const pify = require('pify')

const readFileAsync = pify(fs.readFile)
const router = new Router()

function getPattern (ip = '') {
  if (ip.startsWith('::ffff:')) {
    return ip.replace('::ffff:', '')
  }
  return ip
}

function filter (ip, data) {
  const pattern = getPattern(ip)
  return data
    .split('\n')
    .filter(v => v.includes(`${pattern}#`))
    .join('\n')
}

router.get('/', async ctx => {
  const { ip } = ctx.request
  const data = await readFileAsync('/logs/rpz_log.txt', 'utf8')
  const filteredData = filter(ip, data)
  ctx.body = `ip = ${ip}\n\n${filteredData}`
})

module.exports = router
