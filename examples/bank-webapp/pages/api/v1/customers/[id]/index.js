import axios from 'axios'
import { logError } from 'utils/logger'

export const config = {
  api: {
    bodyParser: false,
  },
}

export default async(req, res) => {
  const {
    query: { id },
    method,
    headers: { authorization },
  } = req
  const url = `${process.env.OXIDE_API_URL}/contacts/${id}`
  switch (method) {
    case 'GET':
      try {
        const customerRes = await axios.get(
          url,
          { headers: { Authorization: authorization } }
        )
        const customer = customerRes.data
        res.status(200).json(customer)
      } catch (e) {
        logError(e)
        const code = e?.response?.data?.code
        res.status(500).json({ error: 'Internal Server Error', code })
      }
      break
    default:
      res.setHeader('Allow', ['GET', 'PUT'])
      res.status(405).end(`Method ${method} Not Allowed`)
  }
}
