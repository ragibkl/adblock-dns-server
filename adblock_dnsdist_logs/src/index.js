const Koa = require('koa')
const Router = require('@koa/router')
const hbs = require('koa-views-handlebars')

const { port } = require('./config')
const { getLogs } = require('./getLogs')

const router = new Router()
router.get('/', getLogs)

const app = new Koa()

app
  .use(hbs(__dirname, {}))
  .use(router.routes())
  .use(router.allowedMethods())

app.listen(port)
