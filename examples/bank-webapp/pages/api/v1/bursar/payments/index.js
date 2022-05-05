import axios from 'axios'
import { logError } from 'utils/logger'

export default async(req, res) => {
  const {
    method,
    headers: { authorization },
  } = req
  const url = ''
  switch (method) {
    case 'GET':
      try {
        const paymentRes = await axios.get(url, { headers: { Authorization: authorization } })
        const payment = paymentRes.data
        res.status(200).json(payment)
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
