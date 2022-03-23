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

class VaultSigner {
  constructor(token, keyName, vaultUrl) {
    this.token = token
    this.key_name = keyName
    this.vault_url = vaultUrl
  }

  async sign(payload) {
    const buffer = Buffer.from(payload)
    const url = `${this.vault_url}/transit/sign/${this.key_name}`
    const res = await axios.post(
      url,
      {
        input: buffer.toString('base64')
      },
      {
        headers: { 'X-Vault-Token': this.token }
      }
    )
    return Buffer.from(res.data.data.signature.split(':', 3)[2], 'base64')
  }

  async getPublicKey() {
    const url = `${this.vault_url}/transit/keys/${this.key_name}`
    const res = await axios.get(url, {
      headers: { 'X-Vault-Token': this.token }
    })
    return Buffer.from(res.data.data.keys['1'].public_key, 'base64')
    // This returns the compressed 32 byte form (I think). That might not be what we want
  }

  getAlgorithm() {
    return Algorithm.ED25519
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
  VaultSigner,
  Algorithm
}
