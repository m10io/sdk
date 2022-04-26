import React from 'react'
import Image from 'next/image'
import PropTypes from 'prop-types'
import classnames from 'classnames'
import styles from './styles/card.module.scss'
import placeholderImage from 'assets/icons/icon-squares-gold.svg'

export const CardHeader = ({ title, icon, iconComponent }) => (
  <div className={styles.cardTitle} id={'card-title'}>
    {icon && (
      <div className={styles.cardTitleIcon}>
        <Image height={20} width={20} src={icon || placeholderImage} />
      </div>
    )}
    {iconComponent && (
      <div className={styles.cardTitleIcon}>
        {iconComponent}
      </div>
    )}
    {title}
  </div>
)

const Card = ({ title, icon, iconComponent, className, children }) => {
  return (
    <div className={classnames(
      styles.cardWrapper,
      className,
    )}>
      {title && <CardHeader title={title} icon={icon} iconComponent={iconComponent} />}
      {children}
    </div>
  )
}

Card.propTypes = {
  title: PropTypes.string,
  icon: PropTypes.object, // imported image (SVG, PNG, etc.)
  className: PropTypes.string,
  iconComponent: PropTypes.node,
}

export default Card
