import axios from 'axios'
import Cookies from 'js-cookie'
import routes from 'routes'

export const getAccounts = async query => (
  await axios.get(routes.ACCOUNTS_API, {
    params: { ...query },
    headers: {
      Authorization: `${Cookies.get('access_token') ? `Bearer ${Cookies.get('access_token')}` : ''}`
    }
  })
)

export const getAccountById = async id => (
  await axios.get(`${routes.ACCOUNTS_API}/${id}`, {
    headers: {
      Authorization: `${Cookies.get('access_token') ? `Bearer ${Cookies.get('access_token')}` : ''}`
    }
  })
)
