/* eslint-disable max-len, no-unused-vars, camelcase, no-return-assign */
import React, { Component, useState, useEffect } from 'react'
import axios from 'axios'
import PropTypes from 'prop-types'
import Page from 'components/page'
import Card from 'components/card'
import routes from 'routes'

class WelcomePage extends Component {
  render() {
    const { windowWidth } = this.props
    return (
      <Page
        withGlobalNav
        windowWidth={windowWidth}
      >
        <Card title={'Unauthorized'}>
          {'You are missing the required roles. Please contact M10 for support.'}
        </Card>
      </Page>
    )
  }
}

export default WelcomePage
