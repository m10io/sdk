const { DirectoryClient } = require('./directory')
const LedgerClient = require('./api')
const crypto = require('./utils/crypto.js')
const utils = require('./utils/utils.js')
class M10 {
    constructor({ address, secure }) {
        this.ledgers = {}
        this.secure = secure
        this.crypto = crypto
        this.utils = utils
        this.directoryClient = new DirectoryClient(address, this.secure)

        return new Proxy(this, {
          get: (target, key) => {
            if (typeof (key) === 'string' && target.ledgers[key] !== undefined) {
              return target.ledgers[key]
            } else {
              return target[key]
            }
          }
      })
    }

    async refreshLedgers(skipRefresh) {
      try {
        const ledgers = await this.directoryClient.listLedgers()

        ledgers.ledgersList.forEach(l => {
          const url = new URL(l.url)
          const disableTls = !(url.protocol === 'https:')
          if (l.operator === 'm10') {
            this.ledgers[`${l.code}.${l.operator}`] =
              new LedgerClient({ host: url.hostname, disableTls: disableTls, instrument: l.code.toUpperCase(), decimals: l.decimals })
          }
        })
      } catch (error) {
        console.log(error)
      }
      if (!skipRefresh) setTimeout(async() => { this.refreshLedgers() }, 10000)
    }
}

module.exports = M10
