const Koa = require('koa')
const cors = require('@koa/cors')
const Router = require('@koa/router')
const hbs = require('koa-views-handlebars')

const { port } = require('./config')
const { getLogs, getLogsApi } = require('./getLogs')

const router = new Router()
router.get('/', getLogs)
router.get('/api/getLogs', getLogsApi)

const app = new Koa()

app
  .use(hbs(__dirname, {}))
  .use(cors())
  .use(router.routes())
  .use(router.allowedMethods())

app.listen(port)
