import axios from 'axios'
import { logError } from 'utils/logger'

export default async(req, res) => {
  const {
    method,
    body,
  } = req
  const url = `${process.env.BASE_API_URL}/oauth/token`
  switch (method) {
    case 'POST':
      try {
        const loginRes = await axios({
          method: 'POST',
          url: url,
          data: {
            client_id: 'bank-emulator',
            grant_type: 'password',
            username: body.username,
            password: body.password,
            audience: process.env.OAUTH_AUDIENCE,
            scope: 'offline_access openid profile email phone',
          }
        })
        res.status(200).json(loginRes.data)
      } catch (e) {
        logError(e)
        res
          .status(e?.response?.status || 500)
          .json({ error: e?.response?.data })
      }
      break
    default:
      res.setHeader('Allow', ['POST'])
      res.status(405).end(`Method ${method} Not Allowed`)
  }
}
