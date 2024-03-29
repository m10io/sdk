import axios from 'axios'
import Cookies from 'js-cookie'
import routes from 'routes'

export const getPaymentById = async id => (
  await axios.get(`${routes.PAYMENTS_API}/${id}`, {
    headers: {
      Authorization: `${Cookies.get('access_token') ? `Bearer ${Cookies.get('access_token')}` : ''}`
    }
  })
)

export const getPaymentsByAsset = async(asset, moneyType) => (
  await axios.get(`${routes.PAYMENTS_API}/assets/${asset}?type=${moneyType}`, {
    headers: {
      Authorization: `${Cookies.get('access_token') ? `Bearer ${Cookies.get('access_token')}` : ''}`
    }
  })
)

export const getPaymentsByAccountIdAndAsset = async(accountId, asset, moneyType) => (
  await axios.get(`${routes.ACCOUNTS_API}/${accountId}/assets/${asset}/payments?type=${moneyType}`, {
    headers: {
      Authorization: `${Cookies.get('access_token') ? `Bearer ${Cookies.get('access_token')}` : ''}`
    }
  })
)

export const getPaymentsByCustomerIdAndAsset = async(customerId, asset) => (
  await axios.get(`${routes.ACCOUNTS_API}/${customerId}/assets/${asset}/payments`, {
    headers: {
      Authorization: `${Cookies.get('access_token') ? `Bearer ${Cookies.get('access_token')}` : ''}`
    }
  })
)
