import axios from 'axios'
import { logError } from 'utils/logger'

export default async(req, res) => {
  const {
    method,
  } = req
  const url = `${process.env.BASE_API_URL}/bursar/api/v1/wallets`
  switch (method) {
    case 'GET':
      try {
        const walletsRes = await axios.get(url)
        const wallets = walletsRes.data
        res.status(200).json(wallets)
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
