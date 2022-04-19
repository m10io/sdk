import React from 'react'
import Container from './container'
import Button, {
  BUTTON_THEME_PRIMARY,
} from './button'
import { Link, useNavigate } from 'react-router-dom'
import m10LogoBlue from '../assets/icons/m10-logo-blue.svg'
import styles from '../styles/global-nav.module.scss'
import routes from '../routes'

const NAV_MENU_ITEMS = [
  {
    title: 'Home',
    url: routes.BASE_ROUTE,
  },
  {
    title: 'Assets',
    url: routes.ASSETS_PAGE_ROUTE,
  },
  {
    title: 'Banks',
    url: routes.ONBOARD_BANK_PAGE_ROUTE,
  },
  {
    title: 'Ledger',
    url: routes.LEDGER_INFO_PAGE_ROUTE,
  },
]

const GlobalNav = ({
  isSmallViewport,
  includeBackButton,
}) => {
  const navigate = useNavigate()
  return (
    <nav role={'navigation'}>
      <div className={styles.navigationWrapper}>
        <Container fullWidth className={styles.navigationContainer}>
          <div className={styles.navigationMenuItemsList}>
            <div className={styles.navigationMenuItemsLeft}>
              <Link to={routes.BASE_ROUTE}>
                <img
                  src={m10LogoBlue}
                  width={100}
                  alt={'m10'}
                />
              </Link>
              <div className={styles.navigationItemsWrapper}>
                {NAV_MENU_ITEMS.map(item => (
                  <Link
                    to={item.url}
                    className={styles.navMenuItem}
                    key={item.url}
                  >
                    {item.title}
                  </Link>
                ))}
              </div>
            </div>
          </div>
          {includeBackButton && (
            <Button
              onClick={() => navigate(-1)}
              theme={BUTTON_THEME_PRIMARY}
              className={styles.backButton}
            >
              {'⬅️'}
            </Button>
          )}
        </Container>
      </div>
    </nav>
  )
}

export default GlobalNav
