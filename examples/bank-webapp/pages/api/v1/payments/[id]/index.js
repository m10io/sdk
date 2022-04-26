import axios from 'axios'
import { logError } from 'utils/logger'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const DEFAULT_ASSET = publicRuntimeConfig.bankAsset

export const config = {
  api: {
    bodyParser: false,
  },
}

export default async(req, res) => {
  const {
    query: { id, accountId, asset },
    method,
    headers: { authorization },
  } = req
  let url = `${process.env.OXIDE_API_URL}/payments/${id}?instrument=${DEFAULT_ASSET.toLowerCase()}`
  if (accountId && asset) {
    url = `${process.env.OXIDE_API_URL}/accounts/${accountId}/assets/${asset}/payments/${id}`
  }
  switch (method) {
    case 'GET':
      try {
        const paymentRes = await axios.get(url, { headers: { Authorization: authorization } })
        const payment = paymentRes.data
        res.status(200).json(payment)
      } catch (e) {
        logError(e)
        res.status(500).json({ error: 'Internal Server Error' })
      }
      break
    default:
      res.setHeader('Allow', ['GET'])
      res.status(405).end(`Method ${method} Not Allowed`)
  }
}
