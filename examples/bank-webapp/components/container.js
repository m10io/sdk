import React from 'react'
import classnames from 'classnames'
import PropTypes from 'prop-types'
import styles from './styles/container.module.scss'

const Container = ({
  children,
  fullWidth,
  className,
  style,
}) => (
  <div
    className={classnames(
      styles.containerWrapper,
      fullWidth && styles.fullWidth,
      className,
    )}
    style={style}
  >
    {children}
  </div>
)

Container.propTypes = {
  children: PropTypes.node.isRequired,
  fullWidth: PropTypes.bool,
  className: PropTypes.string,
  style: PropTypes.object,
}

export default Container
