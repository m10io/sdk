/* eslint-disable camelcase */
import React, { Component } from 'react'
import axios from 'axios'
import PropTypes from 'prop-types'
import Page from 'components/page'
import TableOfflinePayments from 'components/table-offline-payments'
import routes from 'routes'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const CURRENCY = publicRuntimeConfig.bankAsset

class OfflinePaymentsPage extends Component {
  state = {
    isLoading: false,
    loadError: null,
    hasLoaded: false,
    data: [],
  }

  loadTableData = async query => {
    const { hasLoaded, loadError } = this.state
    this.setState({ isLoading: true, loadError: (hasLoaded && loadError) ?? null })
    try {
      const res = await axios.get(`${routes.OFFLINE_PAYMENTS_PAYMENTS_API}?currency=${CURRENCY}`)
      this.setState({ data: res?.data?.data, isLoading: false, hasLoaded: true, loadError: null })
    } catch (e) {
      this.setState({ isLoading: false, loadError: e, hasLoaded: true })
    }
  }

  async componentDidMount() {
    await this.loadTableData()
    this.timerId = setInterval(() => this.loadTableData(), 3000)
  }

  componentWillUnmount() {
    clearInterval(this.timerId)
  }

  render() {
    const {
      isLoading,
      loadError,
      data,
      hasLoaded,
    } = this.state
    const { windowWidth } = this.props
    return (
      <Page
        withGlobalNav
        withSidebar
        loadError={loadError}
        windowWidth={windowWidth}
        navLogoRoute={routes.OFFLINE_PAYMENTS_PAYMENTS_PAGE}
      >
        <TableOfflinePayments
          payments={data || []}
          isLoading={!hasLoaded && isLoading}
          tableName={'Offline Payments'}
          windowWidth={windowWidth}
          loadData={this.loadTableData}
          noPagination
          limit={50}
        />
      </Page>
    )
  }
}

OfflinePaymentsPage.propTypes = {
  query: PropTypes.object,
}

export default OfflinePaymentsPage
