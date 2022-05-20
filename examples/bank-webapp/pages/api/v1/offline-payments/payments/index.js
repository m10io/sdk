import axios from 'axios'
import { logError } from 'utils/logger'

export default async(req, res) => {
  const {
    method,
    query: { currency },
  } = req
  const query = currency ? `?currency=${currency}` : ''
  const url = `${process.env.BASE_API_URL}/bursar/api/v1/payments${query}`
  switch (method) {
    case 'GET':
      try {
        const paymentsRes = await axios.get(url)
        const payments = paymentsRes?.data
        res.status(200).json(payments)
      } catch (e) {
        logError(e)
        const code = e?.response?.data?.code
        res.status(500).json({ error: 'Internal Server Error', code })
      }
      break
    default:
      res.setHeader('Allow', ['GET'])
      res.status(405).end(`Method ${method} Not Allowed`)
  }
}
