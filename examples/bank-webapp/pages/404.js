import React from 'react'
import { useRouter } from 'next/router'
import Image from 'next/image'
import Button, {
  BUTTON_THEME_PRIMARY,
} from 'components/button'
import FourOhFourGraphicComponent from 'assets/404/404.js'
import backgroundImage from 'assets/404/background-image.svg'
import styles from '../components/styles/error-page.module.scss'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

const ErrorPage = () => {
  const router = useRouter()
  return (
    <div>
      <div className={styles.backgroundImage}>
        <Image src={backgroundImage} />
      </div>
      <div className={styles.errorPageContent}>
        <div className={styles.errorPageImage}>
          <FourOhFourGraphicComponent
            color={publicRuntimeConfig.bankPrimaryColor}
          />
        </div>
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
    </div>
  )
}

export default ErrorPage
