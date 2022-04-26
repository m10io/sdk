import axios from 'axios'
import Cookies from 'js-cookie'
import routes from 'routes'

export const getCustomers = async query => (
  await axios.get(routes.CUSTOMERS_API, {
    params: { ...query },
    headers: {
      Authorization: `Bearer ${Cookies.get('access_token')}`
    }
  })
)

export const getCustomerById = async id => (
  await axios.get(`${routes.CUSTOMERS_API}/${id}`, {
    headers: {
      Authorization: `Bearer ${Cookies.get('access_token')}`
    }
  })
)

export const updateCustomerContactInfo = async({ customerId, body }) => (
  await axios({
    method: 'PATCH',
    url: `${routes.CUSTOMERS_API}/${customerId}/contact`,
    data: body,
    headers: {
      Authorization: `Bearer ${Cookies.get('access_token')}`
    },
  })
)
