import { Component } from 'react'
import Router from 'next/router'
import routes, {
  ADMIN_OWNER_ROUTES,
  BUSINESS_OWNER_ROUTES,
  SHARED_AUTHENTICATED_ROUTES,
} from 'routes'
import { cookiesStrToObj } from 'utils/navigation'
import Cookies from 'js-cookie'

export const parseJwt = token => {
  const base64Url = token.split('.')[1]
  const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/')
  const jsonPayload = decodeURIComponent(Buffer.from(base64, 'base64').toString().split('').map(function(c) {
      return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2)
  }).join(''))
  return JSON.parse(jsonPayload)
}

const getAccessToken = req => {
  const cookies = cookiesStrToObj(req.headers.get('cookie'))
  return cookies?.access_token
}

const checkExpiredToken = unixTimestamp => {
  if (unixTimestamp > Date.now()) {
    throw new Error('expired token')
  }
}

const authorizeByRoute = (req, user) => {
  if (!isAuthorizedRoute({ user, route: req.url })) {
    throw new Error(`unauthorized route: ${req.url}`)
  }
}

export const verifyJWT = req => {
  const accessToken = getAccessToken(req)
  if (accessToken) {
    const user = parseJwt(accessToken)
    if (!user) throw new Error('invalid token')
    checkExpiredToken(user)
    authorizeByRoute(req, user)
  } else {
    throw new Error('missing access token')
  }
}

// requires JWT decoded user object
export const determineHomeRouteByUser = user => {
  if (!user) throw new Error('no user')
  const roles = user?.['https://m10.net/roles'] || []
  const isAdmin = isAuthorizedAdmin(roles)
  if (isAdmin) return routes.ADMIN_CUSTOMERS_PAGE
  else return routes.WELCOME_PAGE
}

const ADMIN_ROLE = 'iron-admin'
const TEST_ADMIN_ROLE = 'iron-test-admin'

export const isAuthorizedAdmin = (roles = []) => roles.includes(ADMIN_ROLE) || roles.includes(TEST_ADMIN_ROLE)
// TODO: determine business-owner specific role (i.e. user coming from diapay app to register business)
// NOTE: this will make any valid JWT user devoid of roles able to login to business owner pages (/register)
export const isAuthorizedBusinessOwner = (roles = []) => true /* roles.includes('') */

const isAuthorizedRoute = ({ user, route }) => {
  if (SHARED_AUTHENTICATED_ROUTES.some(r => r === route)) return true
  // TODO: handle other user types by auth0 role
  const roles = user?.['https://m10.net/roles'] || []
  const isAdmin = isAuthorizedAdmin(roles) && ADMIN_OWNER_ROUTES.some(r => route.includes(r))
  const isBusinessOwner = isAuthorizedBusinessOwner(roles) && BUSINESS_OWNER_ROUTES.some(r => r.includes(route))
  return isAdmin || isBusinessOwner
}

export const logout = () => {
  Cookies.set('access_token', '')
  Router.push(routes.LOGIN_PAGE)
}

/**
 * HOC that Redirects an authenticated user to 404 page if not authorized
 */
export function withAuthorization(WrappedComponent) {
  return class extends Component {
    async componentWillMount() {
      const token = Cookies.get('access_token')
      if (!token) Router.push(`${routes.LOGIN_PAGE}?redirect=${Router.route}`)
      const user = parseJwt(token)
      const isAuthorized = isAuthorizedRoute({ user, route: Router.route })
      if (!isAuthorized) Router.replace(routes.ERROR_404_PAGE)
    }

    render() {
      return <WrappedComponent {...this.props} />
    }
  }
}

export const withAuth = Page => withAuthorization(Page)
