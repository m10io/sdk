import React, { useState } from 'react'
import classnames from 'classnames'
import PropTypes from 'prop-types'
import routes from 'routes'
import Image from 'next/image'
import iconArrowUpLightGrey from 'assets/icons/icon-arrow-up-blue.svg'
import IconUsers from 'assets/icons/icon-users'
import IconBookOpen from 'assets/icons/icon-book-open'
import IconDollarSign from 'icons/icon-dollar-sign'
import Link from './link'
import styles from './styles/sidebar.module.scss'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

export const createNavItems = () => {
  return [
    {
      title: 'Payments',
      url: routes.ADMIN_PAYMENTS_PAGE,
      icon: <IconDollarSign color={publicRuntimeConfig.bankPrimaryColor} />,
    },
    {
      title: 'Customers',
      url: routes.ADMIN_CUSTOMERS_PAGE,
      icon: <IconUsers color={publicRuntimeConfig.bankPrimaryColor} />,
  },
    {
      title: 'Offline Payments',
      url: routes.OFFLINE_PAYMENTS_PAYMENTS_PAGE,
      icon: <IconDollarSign color={publicRuntimeConfig.bankPrimaryColor} />,
  },
    {
      title: 'Accounts',
      url: routes.ADMIN_ACCOUNTS_PAGE,
      icon: <IconBookOpen color={publicRuntimeConfig.bankPrimaryColor} />,
    },
  ]
}

const Sidebar = ({ router, customer, jwtUser, isLoading }) => {
  const NAV_ITEMS = createNavItems()
  const { pathname, asPath } = router

  // auto-open submenu if it includes current page route
  const setInitialState = () => {
    const openSubmenusByIndex = new Set()
    NAV_ITEMS.forEach((item, index) => {
      // NOTE: all submenus open in sidebar initial state
      openSubmenusByIndex.add(index)
    })
    return { openSubmenusByIndex }
  }
  const [sidebarMenuOpenState, setSidebarMenuOpenState] = useState(setInitialState())
  const toggleMenuOpenState = index => {
    setSidebarMenuOpenState(prevState => {
      const openSubmenusByIndexUpdated = prevState.openSubmenusByIndex
      prevState.openSubmenusByIndex.has(index) // eslint-disable-line no-unused-expressions
        ? openSubmenusByIndexUpdated.delete(index)
        : openSubmenusByIndexUpdated.add(index)
      return { ...prevState, openSubmenusByIndex: openSubmenusByIndexUpdated }
    })
  }

  // NOTE: handle sidebar item hoverState dynamically via item index
  const [hoverState, setHoverState] = useState(Object.values(NAV_ITEMS).map(v => false))
  const setHoverStateByIndex = index => {
    const newHoverState = hoverState.map((v, i) => i === index)
    setHoverState(newHoverState)
  }
  const removeHoverState = () => setHoverState(hoverState.map(v => false))

  return (
    <div className={styles.sidebarWrapper} id={'sidebar'}>
      <div className={styles.sidebarContainer}>
        {NAV_ITEMS.map((item, index) => {
          const subMenuItems = item.subMenuItems || []
          const isSubmenuOpen = sidebarMenuOpenState.openSubmenusByIndex.has(index)
          const isCurrentMenuItemPath = item.url === pathname || pathname.includes(item.url)
          const isCurrentMenuItemRoot = (item.highlightedRoutes || []).some(route => (pathname.includes(route) || asPath.includes(route)))
          const isHovered = !!hoverState[index]
          return (
            <div key={item.title}>
              <Link
                href={isCurrentMenuItemRoot && item.preventLinkIfRoot ? null : item.url}
                onClick={item.subMenuItems ? () => toggleMenuOpenState(index) : () => {}}
                className={classnames(
                  styles.sidebarMenuItemWrapper,
                  isHovered && styles.sidebarMenuItemActive,
                  styles.sidebarMenuItemTitle,
                  isSubmenuOpen && styles.sidebarMenuItemTitleWithOpenSubmenu,
                  (isCurrentMenuItemPath || isCurrentMenuItemRoot) && styles.sidebarMenuItemActive,
                )}
                onMouseOver={() => setHoverStateByIndex(index)} onMouseLeave={removeHoverState}
                disabled={isCurrentMenuItemRoot && item.preventLinkIfRoot}
              >
                <div className={styles.sidebarMenuItemContainer}>
                  {item.icon}
                  <div className={styles.sidebarMenuIconTitle}>
                    {item.title}
                  </div>
                </div>
                {!!item.subMenuItems && (
                  <div className={styles.submenuToggleArrow}>
                    <Image
                      src={iconArrowUpLightGrey}
                      alt={isSubmenuOpen ? 'Close' : 'Open'}
                    />
                  </div>
                )}
              </Link>
              {isSubmenuOpen ? (
                <div className={styles.sidebarsubMenuItemsWrapper}>
                  {subMenuItems.map(submenuItem => {
                    const isCurrentSubMenuItemPath = (submenuItem.url === pathname || asPath.includes(submenuItem.url))
                    const isSubmenuItemHighlightedRoute = (submenuItem.highlightedRoutes || []).some(r => r.includes(pathname))
                    return (
                      <Link
                        className={classnames(
                          styles.sidebarMenuItemWrapper,
                          styles.sidebarSubmenuItemWrapper,
                          (isCurrentSubMenuItemPath || isSubmenuItemHighlightedRoute) && styles.sidebarMenuItemActive,
                        )}
                        href={submenuItem.url}
                        key={submenuItem.title}
                      >
                        {submenuItem.title}
                      </Link>
                    )
                  })}
                </div>
              ) : null}
            </div>
          )
        })}
      </div>
    </div>
  )
}

Sidebar.propTypes = {
  router: PropTypes.shape({
    pathname: PropTypes.string.isRequired,
    asPath: PropTypes.string,
  }).isRequired,
  customer: PropTypes.object,
}

export default Sidebar
