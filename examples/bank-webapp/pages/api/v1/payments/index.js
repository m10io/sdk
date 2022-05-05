/* eslint-disable camelcase */
import axios from 'axios'
import { logError } from 'utils/logger'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const DEFAULT_ASSET = publicRuntimeConfig.bankAsset

export default async(req, res) => {
  const {
    method,
    headers: { authorization },
    query: {
      id,
      time,
      limit,
      customer_id,
      business_id,
    },
  } = req
  const url = `${process.env.OXIDE_API_URL}/payments?instrument=${DEFAULT_ASSET.toLowerCase()}`
  switch (method) {
    case 'GET':
      try {
        let fetchUrl = `${url}&limit=${limit || 10}${id && time ? `&id=${id}&time=${time}` : ''}`
        if (customer_id) fetchUrl += `&customer_id=${customer_id}`
        if (business_id) fetchUrl += `&business_id=${business_id}`
        const paymentsRes = await axios.get(fetchUrl, { headers: { Authorization: authorization } })
        const payments = paymentsRes.data
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
