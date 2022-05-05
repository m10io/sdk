/* eslint-disable camelcase */
import { NextResponse } from 'next/server'
import { verifyJWT } from 'utils/auth'
import routes from 'routes'

const isAuthenticatedRoute = url => {
  return routes.AUTHENTICATED_ROUTES.some(route => url.includes(routes))
}

export function middleware(req, res) {
  try {
    if (!isAuthenticatedRoute(req.nextUrl.pathname)) NextResponse.next()
    else {
      verifyJWT(req)
      NextResponse.next()
    }
  } catch (e) {
    return NextResponse.redirect(`${routes.LOGIN_PAGE}?redirect=${req.url}`)
  }
}
