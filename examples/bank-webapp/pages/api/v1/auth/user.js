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
  const url = `${process.env.BASE_API_URL}/oauth/userinfo`
  switch (method) {
    case 'GET':
      try {
        const userInfoRes = await axios({
          method: 'GET',
          url: url,
          headers: { Authorization: authorization },
          data: {},
        })
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
