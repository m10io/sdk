import React, { Component } from 'react'
import classnames from 'classnames'
import { withRouter } from 'next/router'
import PropTypes from 'prop-types'
import Cookies from 'js-cookie'
import Container from './container'
import GlobalNav from './global-nav'
import Sidebar from './sidebar'
import Button from 'components/button'
import LoadingOverlay from 'components/loading-overlay'
import { logout, parseJwt } from 'utils/auth'
import { getUser } from 'lib/api/auth'
import routes from 'routes'
import styles from './styles/page.module.scss'

const redirectIfUnauthorized = (nextProps, router) => {
  const code = nextProps?.loadError?.response?.data?.code
  const isUnauthorized = code === 'unauthorized'
  if (isUnauthorized) {
    router.push(`${routes.LOGIN_PAGE}?redirect=${router.asPath}`)
  }
}

export const PageError = ({ router }) => {
  return (
    <div className={styles.pageErrorWrapper}>
      <div className={styles.pageErrorTitleText}>
        {'Failed to load data'}
      </div>
      <Button
        onClick={() => router.reload()}
        text={'Reload'}
      />
    </div>
  )
}

class Page extends Component {
  state = {
    jwtUser: {},
    auth0User: {},
  }

  getUserInfo = async token => {
    try {
      const userInfoRes = await getUser()
      return userInfoRes?.data
    } catch (e) {
      // TODO: handle unauthorized error across all requests
      if (e?.response?.data === 'Unauthorized') {
        logout()
      } else {
        console.log(e)
      }
    }
  }

  async componentDidMount() {
    try {
      const token = Cookies.get('access_token')
      if (token) {
        const jwtUser = parseJwt(token)
        const auth0User = await this.getUserInfo(token)
        this.setState({ jwtUser, auth0User })
      }
    } catch (e) {
      console.log(e)
    }
  }

  componentWillReceiveProps(nextProps) {
    redirectIfUnauthorized(nextProps, this.props.router)
  }

  render() {
    const { jwtUser, auth0User } = this.state
    const {
      children,
      router,
      fullWidth,
      loadError,
      wrapperClassName,
      containerClassName,
      withSidebar,
      withGlobalNav,
      windowWidth,
      customer,
      isLoading,
      withM10Logo,
      centeredContent,
      navLogoRoute,
    } = this.props
    const isSmallViewport = windowWidth < 900
    return (windowWidth
      ? (
        <div>
          {withGlobalNav && (
            <GlobalNav
              router={router}
              isSmallViewport={isSmallViewport}
              user={auth0User}
              logout={logout}
              customer={customer}
              jwtUser={jwtUser}
              withM10Logo={withM10Logo}
              navLogoRoute={navLogoRoute}
            />
          )}
          <div className={classnames(
            styles.pageWrapper,
            wrapperClassName,
          )}>
            {withSidebar && !isSmallViewport && (
              <Sidebar router={router} customer={customer} jwtUser={jwtUser} />
            )}
            <div className={classnames(
              styles.pageContainer,
              fullWidth && styles.pageContainerFullWidth,
              containerClassName,
            )}>
              <Container fullWidth={fullWidth} centeredContent={centeredContent}>
                {isLoading
                  ? <LoadingOverlay />
                  : loadError
                    ? <PageError router={router} />
                    : children
                }
              </Container>
            </div>
          </div>
        </div>
      ) : null
    )
  }
}

Page.propTypes = {
  children: PropTypes.node,
  router: PropTypes.shape({
    pathname: PropTypes.string.isRequired,
  }).isRequired,
  fullWidth: PropTypes.bool,
  loadError: PropTypes.oneOfType([
    PropTypes.object,
    PropTypes.bool,
  ]),
  wrapperClassName: PropTypes.string,
  containerClassName: PropTypes.string,
  withSidebar: PropTypes.bool,
  customer: PropTypes.object,
  isLoading: PropTypes.bool,
  title: PropTypes.string,
  bannerText: PropTypes.string,
  centeredContent: PropTypes.bool,
  withM10Logo: PropTypes.bool,
  navLogoRoute: PropTypes.string,
}

export default withRouter(Page)
