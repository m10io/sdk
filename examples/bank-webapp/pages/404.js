import React from 'react'
import { useRouter } from 'next/router'
import Page from 'components/page'
import Image from 'next/image'
import Button, {
  BUTTON_THEME_PRIMARY,
} from 'components/button'
import FourOhFourGraphic from 'assets/404/404.svg'
import backgroundImage from 'assets/404/background-image.svg'
import styles from '../components/styles/error-page.module.scss'

const ErrorPage = () => {
  const router = useRouter()
  return (
    <Page
      fullWidth
      wrapperClassName={styles.pageWrapper404}
      withGlobalNav
    >
      <div className={styles.backgroundImage}>
        <Image src={backgroundImage} />
      </div>
      <div className={styles.errorPageContent}>
        <Image
          src={FourOhFourGraphic}
          alt={'404'}
          className={styles.errorPageImage}
        />
        <div className={styles.errorPageText}>
          {'We can\'t seem to find the page you\'re looking for'}
        </div>
        <Button
          theme={BUTTON_THEME_PRIMARY}
          text={'Go Back'}
          onClick={() => router.back()}
          className={styles.errorPageButton}
        />
      </div>
    </Page>
  )
}

export default ErrorPage
