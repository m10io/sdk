import React, { useState } from 'react'
import PropTypes from 'prop-types'
import MobileNav from './mobile-nav'
import Image from 'next/image'
import Link from './link'
import OutsideClickAlerter from 'components/outside-click-alerter'
import authPlaceholderIcon from 'assets/icons/icon-default-user-circle.svg'
import logoutIcon from 'assets/icons/icon-logout.svg'
import { createNavItems } from 'components/sidebar'
import routes from 'routes'
import styles from './styles/global-nav.module.scss'

const GlobalNavDropdownToggle = ({ imgSrc }) => (
  <div className={styles.globalNavDropdownToggleIcon}>
    <Image
      src={imgSrc || authPlaceholderIcon}
      alt={'Settings'}
      className={styles.dropdownLogoutIcon}
      id={'navigation-menu-dropdown-logout-icon'}
      height={50}
      width={50}
    />
  </div>
)

GlobalNavDropdownToggle.propTypes = {
  imgSrc: PropTypes.string,
}

const GlobalNavDropdownMenu = ({
  user,
  isVisible,
  logout,
  customer,
}) => {
  const name = customer?.data?.name || user?.name
  return (
    <div
      className={isVisible ? styles.fadeIn : styles.fadeOut}
      id={'global-nav-dropdown-menu'}
    >
      <div className={`${styles.globalNavDropdown} ${styles.globalNavText}`}>
        <Link href={routes.PROFILE_PAGE} className={styles.profileInfoLink}>
          <div className={styles.dropdownName}>{name}</div>
          <div className={styles.dropdownEmail}>{user.email}</div>
        </Link>
        <div className={styles.dropdownDivider} />
        <div
          className={styles.dropdownSignOut}
          onClick={() => logout({ returnTo: window.location.origin })}
        >
          <span className={styles.logoutIcon}>
            <Image src={logoutIcon} height={24} width={24} />
          </span>
          {'Sign out'}
        </div>
      </div>
    </div>
  )
}

GlobalNavDropdownMenu.propTypes = {
  user: PropTypes.shape({
    name: PropTypes.string,
    email: PropTypes.string,
  }),
  isVisible: PropTypes.bool,
  customer: PropTypes.shape({
    data: PropTypes.shape({
      name: PropTypes.string,
    })
  }),
}

const LOGGED_IN_BASE_ROUTE = routes.ADMIN_CUSTOMERS_PAGE

const GlobalNav = ({
  navMenuItems = [],
  router,
  isSmallViewport,
  user,
  logout,
  customer = {},
  jwtUser,
  bannerText,
  withM10Logo,
  navLogoRoute,
}) => {
  const [isShowingMenu, setShowingMenu] = useState(false)
  const handleClickOutside = () => setShowingMenu(false)
  const NAV_ITEMS = createNavItems({ isRegistered: !!customer?.business_id, jwtUser }) // eslint-disable-line
  return (
    <>
      {isSmallViewport
        ? (
          <MobileNav
            menuItems={NAV_ITEMS}
            user={user}
            logout={logout}
            navLogoRoute={navLogoRoute}
            LOGGED_IN_BASE_ROUTE={LOGGED_IN_BASE_ROUTE}
            withM10Logo={withM10Logo}
          />
        ) : <div>
          {user?.sub && (
            <OutsideClickAlerter
              onOutsideClick={handleClickOutside}
              shouldTrackClick
              elementIdsToIgnore={[
                'navigation-menu-dropdown-logout-icon',
                'navigation-menu-items-right',
              ]}
            >
              <GlobalNavDropdownMenu
                isVisible={isShowingMenu}
                user={user}
                customer={customer}
                logout={logout}
              />
            </OutsideClickAlerter>
          )}
        </div>
      }
    </>
  )
}

GlobalNav.propTypes = {
  navMenuItems: PropTypes.arrayOf(PropTypes.object),
  router: PropTypes.object,
  isSmallViewport: PropTypes.bool,
  user: PropTypes.shape({
    name: PropTypes.string,
    email: PropTypes.string,
  }),
  jwtUser: PropTypes.shape({
    permissions: PropTypes.arrayOf(PropTypes.string.isRequired),
  }),
  withM10Logo: PropTypes.bool,
  navLogoRoute: PropTypes.string,
}

export default GlobalNav
