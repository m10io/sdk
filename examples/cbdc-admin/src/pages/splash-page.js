import Page from '../components/page'
import classnames from 'classnames'
import Card from '../components/card'
import Button, {
  BUTTON_THEME_PRIMARY,
} from '../components/button'
import { Link } from 'react-router-dom'
import routes from '../routes'
import styles from '../styles/shared.module.scss'

function SplashPage() {
  return (
    <div>
      <header className={styles.app}>
        <Page withGlobalNav>
          <Card title={'CBDC Control Panel'}>
            <>
              <Link to={routes.ASSETS_PAGE_ROUTE}>
                <Button
                  theme={BUTTON_THEME_PRIMARY}
                  text={'New Asset'}
                  className={classnames(
                    styles.clickableDiv,
                    styles.splashPageButton,
                  )}
                />
              </Link>
              <Link to={routes.ONBOARD_BANK_PAGE_ROUTE}>
                <Button
                  theme={BUTTON_THEME_PRIMARY}
                  text={'Onboard Bank'}
                  className={classnames(
                    styles.clickableDiv,
                    styles.splashPageButton,
                  )}
                />
              </Link>
              <Link to={routes.ISSUE_PAGE_ROUTE}>
                <Button
                  theme={BUTTON_THEME_PRIMARY}
                  text={'Issue'}
                  className={classnames(
                    styles.clickableDiv,
                    styles.splashPageButton,
                  )}
                />
              </Link>
              <Link to={routes.REDEEM_PAGE_ROUTE}>
                <Button
                  theme={BUTTON_THEME_PRIMARY}
                  text={'Redeem'}
                  className={classnames(
                    styles.clickableDiv,
                    styles.splashPageButton,
                  )}
                />
              </Link>
            </>
          </Card>
        </Page>
      </header>
    </div>
  )
}

export default SplashPage
