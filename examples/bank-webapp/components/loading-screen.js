import React from 'react'
import LoadingOverlay from 'components/loading-overlay'
import Image from 'next/image'
import diaPayLogo from '../assets/logos/diapay-blue.svg'
import styles from 'components/styles/loading-overlay.module.scss'

const LoadingScreen = ({ windowWidth }) => {
  return (
    <div className={styles.fullscreenLoaderContainer}>
      <div className={styles.loadingScreenItems}>
        <div className={styles.diaPayLogo}>
          <Image src={diaPayLogo} alt={'M10'} />
        </div>
        <LoadingOverlay />
      </div>
    </div>
  )
}

export default LoadingScreen
