/* eslint-disable camelcase */
import axios from 'axios'
import { logError } from 'utils/logger'

export const config = {
  api: {
    bodyParser: false,
  },
}

export default async(req, res) => {
  const {
    method,
    headers: { authorization },
    query: {
      id,
      time,
      limit,
    },
  } = req
  const url = `${process.env.OXIDE_API_URL}/contacts`
  switch (method) {
    case 'GET':
      try {
        const fetchUrl = `${url}?limit=${limit || 50}${id && time ? `&id=${id}&time=${time}` : ''}`
        const data = await axios.get(fetchUrl, { headers: { Authorization: authorization } })
        const customers = data.data
        res.status(200).json(customers)
      } catch (e) {
        logError(e)
        const code = e?.response?.data?.code
        res.status(500).json({ error: 'Internal Server Error', code })
      }
      break
    default:
      res.setHeader('Allow', ['GET'])
      res.status(405).end(`Method ${method} Not Allowed`)
      break
  }
}
