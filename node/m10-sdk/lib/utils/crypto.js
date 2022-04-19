const crypto = require('crypto')
const axios = require('axios')

class CryptoSigner {
  constructor(privateKey) {
    this.privateKey = crypto.createPrivateKey(privateKey)
    this.publicKey = crypto.createPublicKey(privateKey)
    if (this.privateKey.asymmetricKeyType === 'ed25519') {
      this.algorithm = Algorithm.ED25519
    }
  }

  async sign(payload) {
    return crypto.sign(null, Buffer.from(payload), this.privateKey)
  }

  async getPublicKey() {
    const publicKey = this.publicKey.export({ type: 'spki', format: 'der' })
    // remove header (first 12 bytes) from buffer
    return publicKey.slice(12)
  }

  getAlgorithm() {
    return this.algorithm
  }

  static generateKeyPair() {
    const { privateKey } = crypto.generateKeyPairSync('ed25519')
    return new CryptoSigner(privateKey.export({ type: 'pkcs8', format: 'pem' }))
  }
}

class CryptoHasher {
  constructor() {
    this.hasher = crypto.createHash('sha256')
  }

  hash(payload) {
    return this.hasher.digest(Buffer.from(payload))
  }
}

const Algorithm = {
  ED25519: 1
}

module.exports = {
  CryptoSigner,
  CryptoHasher,
  Algorithm
}
