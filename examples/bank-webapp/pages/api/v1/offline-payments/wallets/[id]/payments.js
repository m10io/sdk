import axios from 'axios'
import { logError } from 'utils/logger'

export default async(req, res) => {
  const {
    method,
    query: { id },
  } = req
  const url = `${process.env.BASE_API_URL}/bursar/api/v1/wallets/${id}/payments`
  switch (method) {
    case 'GET':
      try {
        const walletPaymentsRes = await axios.get(url)
        const payments = walletPaymentsRes.data
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
