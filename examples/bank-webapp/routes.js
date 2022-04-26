// page routes
const BASE_ROUTE = '/'
const LOGIN_PAGE = '/login'
const ADMIN_PAGE = '/admin'
const WELCOME_PAGE = '/welcome'
const ERROR_404_PAGE = '/404'
const ERROR_500_PAGE = '/500'
const PROFILE_PAGE = '/profile'

// admin
const ADMIN_CUSTOMERS_PAGE = '/admin/customers'
const ADMIN_PAYMENTS_PAGE = '/admin/payments'
const ADMIN_ACCOUNTS_PAGE = '/admin/accounts'

// external page routes
const FAQ_PAGE = '#'
const SUPPORT_PAGE = '#'

// api routes
const CUSTOMERS_API = '/api/v1/customers'
const ACCOUNTS_API = '/api/v1/accounts'
const PAYMENTS_API = '/api/v1/payments'
const AUTH_API = '/api/v1/auth'

const UNAUTHENTICATED_ROUTES = [
  BASE_ROUTE,
  ERROR_404_PAGE,
  ERROR_500_PAGE,
  LOGIN_PAGE,
]

const ADMIN_OWNER_ROUTES = [
  ADMIN_CUSTOMERS_PAGE,
  ADMIN_PAYMENTS_PAGE,
]

const BUSINESS_OWNER_ROUTES = [
  WELCOME_PAGE,
]

const SHARED_AUTHENTICATED_ROUTES = [
  PROFILE_PAGE,
  CUSTOMERS_API,
  ACCOUNTS_API,
  PAYMENTS_API,
  AUTH_API,
]

const AUTHENTICATED_ROUTES = [
  ...ADMIN_OWNER_ROUTES,
  ...BUSINESS_OWNER_ROUTES,
  ...SHARED_AUTHENTICATED_ROUTES,
]

module.exports = {
  // pages
  ERROR_404_PAGE,
  ERROR_500_PAGE,
  BASE_ROUTE,
  LOGIN_PAGE,
  PROFILE_PAGE,

  // business owner
  ADMIN_PAGE,
  WELCOME_PAGE,

  // bank admin
  ADMIN_CUSTOMERS_PAGE,
  ADMIN_PAYMENTS_PAGE,
  ADMIN_ACCOUNTS_PAGE,

  // external pages
  FAQ_PAGE,
  SUPPORT_PAGE,

  // api
  CUSTOMERS_API,
  ACCOUNTS_API,
  PAYMENTS_API,
  AUTH_API,

  // groupings
  UNAUTHENTICATED_ROUTES,
  ADMIN_OWNER_ROUTES,
  BUSINESS_OWNER_ROUTES,
  AUTHENTICATED_ROUTES,
  SHARED_AUTHENTICATED_ROUTES,
}
