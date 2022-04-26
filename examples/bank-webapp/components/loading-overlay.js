import React from 'react'
import classnames from 'classnames'
import LoadingSpinnerIcon from 'assets/icons/loading-spinner'
import styles from './styles/loading-overlay.module.scss'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

const LoadingOverlay = ({ className }) => {
  return (
    <div className={classnames(
      styles.blockContainer,
      className,
    )}>
      {LoadingSpinner}
    </div>
  )
}

export const LoadingSpinner = (
  <div className={styles.loadingSpinner}>
    <LoadingSpinnerIcon color={publicRuntimeConfig.bankPrimaryColor} />
  </div>
)

export default LoadingOverlay
