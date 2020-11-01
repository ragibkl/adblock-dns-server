const Koa = require('koa')
const router = require('./routes')

const app = new Koa()

app
  .use(router.routes())
  .use(router.allowedMethods())

app.listen(8080, '0.0.0.0')
