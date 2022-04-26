import React, { useState } from 'react'
import PropTypes from 'prop-types'
import classnames from 'classnames'
import Container from './container'
import MobileNav from './mobile-nav'
import Image from 'next/image'
import Link from './link'
import OutsideClickAlerter from 'components/outside-click-alerter'
import authPlaceholderIcon from 'assets/icons/icon-default-user-circle.svg'
import logoutIcon from 'assets/icons/icon-logout.svg'
import { createNavItems } from 'components/sidebar'
import routes from 'routes'
import styles from './styles/global-nav.module.scss'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

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

const GlobalNav = ({
  navMenuItems = [],
  router,
  isSmallViewport,
  user,
  logout,
  customer = {},
  jwtUser,
}) => {
  const [isShowingMenu, setShowingMenu] = useState(false)
  const toggleMenu = () => setShowingMenu(!isShowingMenu)
  const handleClickOutside = () => setShowingMenu(false)
  const NAV_ITEMS = createNavItems({ isRegistered: !!customer?.business_id, jwtUser }) // eslint-disable-line
  return (
    <nav role={'navigation'}>
      {isSmallViewport
        ? <MobileNav menuItems={NAV_ITEMS} user={user} logout={logout} />
        : (
          <div className={styles.navigationWrapper}>
            <Container fullWidth className={styles.navigationContainer}>
              <div className={styles.navigationMenuItemsList}>
                <div className={styles.navigationMenuItemsLeft}>
                  <Link
                    className={classnames(
                      styles.navigationItem,
                      styles.navigationLogo,
                    )}
                    href={'/'}
                  >
                    <div className={styles.bankLogoName} style={{ background: publicRuntimeConfig.bankPrimaryColor }}>
                      {publicRuntimeConfig.bankName}
                    </div>
                  </Link>
                  <div className={styles.navigationItemsWrapper}>
                    {/* nav menu items */}
                  </div>
                </div>
                {user?.sub && (
                  <div className={styles.navigationMenuItemsRight}>
                    <div className={styles.navigationMenuItemRight}>
                    </div>
                    <div
                      className={styles.navigationMenuItemRight}
                      onClick={toggleMenu}
                      id={'navigation-menu-items-right'}
                    >
                      <GlobalNavDropdownToggle imgSrc={user?.picture} />
                    </div>
                  </div>
                )}
              </div>
            </Container>
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
        )}
    </nav>
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
  customer: PropTypes.shape({
    business_id: PropTypes.number,
  }),
  jwtUser: PropTypes.shape({
    permissions: PropTypes.arrayOf(PropTypes.string.isRequired),
  }),
}

export default GlobalNav
