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
  } = req
  const url = `${process.env.OAUTH_DOMAIN}/userinfo`
  switch (method) {
    case 'GET':
      try {
        const userInfoRes = await axios.get(url, { headers: { Authorization: authorization } })
        const userInfo = userInfoRes.data
        res.status(200).json(userInfo)
      } catch (e) {
        logError(e)
        res.status(500).send(e?.response?.data)
      }
      break
    default:
      res.setHeader('Allow', ['GET'])
      res.status(405).end(`Method ${method} Not Allowed`)
  }
}
