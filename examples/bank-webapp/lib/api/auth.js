import axios from 'axios'
import Cookies from 'js-cookie'
import routes from 'routes'

export const login = async body => (
  await axios({
    method: 'POST',
    url: `${routes.AUTH_API}/login`,
    data: body,
  })
)

export const getUser = async() => (
  await axios.get(`${routes.AUTH_API}/user`, {
    headers: {
      Authorization: `${Cookies.get('access_token') ? `Bearer ${Cookies.get('access_token')}` : ''}`,
    },
  })
)
