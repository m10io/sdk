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
    body,
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
        res.status(500).json({ error: 'Internal Server Error' })
      }
      break
      case 'PUT':
        try {
          const updateCustomerRes = await axios({
            method: 'POST',
            url: url,
            data: body,
            headers: {
              'Content-Type': 'application/json',
              Authorization: authorization,
            },
          })
          res.status(200).json(updateCustomerRes.data)
        } catch (e) {
          logError(e)
          res.status(500).json({ error: 'Internal Server Error' })
        }
        break
    default:
      res.setHeader('Allow', ['GET', 'PUT'])
      res.status(405).end(`Method ${method} Not Allowed`)
  }
}
