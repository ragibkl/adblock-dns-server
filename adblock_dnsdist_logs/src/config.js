const logFile = process.env.LOG_FILE || './logs.yaml'
const port = parseInt(process.env.PORT || '8080', 10)

module.exports = {
  logFile,
  port
}
