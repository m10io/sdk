const {
  v4: uuidv4,
  stringify: uuidStringify,
  parse: uuidParse,
  validate: uuidValidate,
} = require('uuid')
const btoa = require('btoa')
const atob = require('atob')

const required = name => {
  throw new Error(`Parameter ${name} is required`)
}

function newId() {
  const id = []
  uuidv4(null, id)
  return new Uint8Array(id)
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

function isUuid(id) {
  return uuidValidate(id)
}

function verifyUuid(id) {
  if (!isUuid(id)) {
    throw new Error('Invalid id')
  }
}

const hexRegex = /[0-9A-Fa-f]{6}/
function isHexStr(str) {
  return hexRegex.test(str)
}

function verifyHexStr(str) {
  if (!isHexStr(str)) {
    throw new Error('Invalid id')
  }
}

const base64regex = /^([0-9a-zA-Z+/]{4})*(([0-9a-zA-Z+/]{2}==)|([0-9a-zA-Z+/]{3}=))?$/
function isBase64Str(str) {
  return base64regex.test(str)
}

function verifyBase64Str(str) {
  if (!isBase64Str(str)) {
    throw new Error('Invalid id')
  }
}

function uint8ArrayToUuid(uint8arr) {
  return uint8arr instanceof Uint8Array ? uuidStringify(uint8arr) : null
}

function uint8ArrayToBase64(uint8arr) {
  return btoa(String.fromCharCode(...new Uint8Array(uint8arr)))
}

function uint8ArrayToBase64Str(uint8arr) {
  return uint8arr instanceof Uint8Array ? uint8ArrayToBase64(uint8arr) : null
}

function uint8ArrayToHexStr(uint8arr) {
  return uint8arr instanceof Uint8Array
    ? Buffer.from(uint8arr).toString('hex')
    : null
}

function parseHexStrToUint8Array(hexStr) {
  return new Uint8Array(Buffer.from(hexStr, 'hex'))
}

function parseBase64StrToUint8Array(base64Str) {
  return Uint8Array.from(atob(base64Str), c => c.charCodeAt(0))
}

const Hex = {
  // Throws an exception if the string is not hex
  verify: verifyHexStr,
  // Returns true if the string is hex
  isValid: isHexStr,
  // Converts the hex string to a byte array
  toUint8Array: parseHexStrToUint8Array,
  // Converts the byte array to a hex string
  fromUint8Array: uint8ArrayToHexStr,
}

const Base64 = {
  // Throws an exception if the string is not base64
  verify: verifyBase64Str,
  // Returns true if the string is base64
  isValid: isBase64Str,
  // Converts the base64 string to a byte array
  toUint8Array: parseBase64StrToUint8Array,
  // Converts the byte array to a base64 string
  fromUint8Array: uint8ArrayToBase64Str,
}

const Uuid = {
  // Generates a new UUIDv4
  newId: newId,
  // Throws an exception if the string is not a UUID
  verify: verifyUuid,
  // Returns true if the string is a UUID
  isValid: isUuid,
  // Converts the UUID into a byte array
  toUint8Array: uuidParse,
  // Converts the byte array to a UUID
  fromUint8Array: uint8ArrayToUuid,
}

module.exports = {
  required,
  sleep,
  verifyBase64Str,
  uint8ArrayToBase64,
  Hex,
  Base64,
  Uuid,
}
