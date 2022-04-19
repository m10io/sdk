import React, { Component } from 'react'
import classnames from 'classnames'
import PropTypes from 'prop-types'
import Container from './container'
import GlobalNav from './global-nav'
// import LoadingOverlay from 'components/loading-overlay'
import styles from '../styles/page.module.scss'

class Page extends Component {
  render() {
    const {
      children,
      router,
      fullWidth,
      wrapperClassName,
      containerClassName,
      withGlobalNav,
      windowWidth,
      // isLoading,
      title,
      includeBackButton,
    } = this.props
    const isSmallViewport = windowWidth < 900
    return (
      <div>
        {withGlobalNav && (
          <GlobalNav
            router={router}
            isSmallViewport={isSmallViewport}
            includeBackButton={includeBackButton}
          />
        )}
        <div className={classnames(
          styles.pageWrapper,
          wrapperClassName,
        )}>
          <div className={classnames(
            styles.pageContainer,
            fullWidth && styles.pageContainerFullWidth,
            containerClassName,
          )}>
            <Container fullWidth={fullWidth}>
              {title && <h2 className={styles.pageTitle}>{title}</h2>}
              {children}
              {/* isLoading && <LoadingOverlay /> */}
            </Container>
          </div>
        </div>
      </div>
    )
  }
}

Page.propTypes = {
  children: PropTypes.node,
  fullWidth: PropTypes.bool,
  wrapperClassName: PropTypes.string,
  containerClassName: PropTypes.string,
  isLoading: PropTypes.bool,
  title: PropTypes.string,
  includeBackButton: PropTypes.bool,
}

export default Page
