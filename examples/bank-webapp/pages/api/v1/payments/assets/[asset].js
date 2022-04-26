/* eslint-disable camelcase */
import axios from 'axios'
import { logError } from 'utils/logger'

export default async(req, res) => {
  const {
    method,
    headers: { authorization },
    query: {
      asset,
      limit,
    },
  } = req
  const url = `${process.env.OXIDE_API_URL}/assets/${asset}/payments?limit=${limit || 10}`
  switch (method) {
    case 'GET':
      try {
        const paymentsRes = await axios.get(url, { headers: { Authorization: authorization } })
        const payments = paymentsRes.data
        res.status(200).json(payments)
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
