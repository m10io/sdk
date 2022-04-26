/* eslint-disable camelcase */
import React, { Component } from 'react'
import PropTypes from 'prop-types'
import Page from 'components/page'
import { withRouter } from 'next/router'
import TablePayments from 'components/table-payments'
import { getPaymentsByAsset } from 'lib/api/payments'
import { parsePaymentsApi } from 'utils/api'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const BANK_ASSET = publicRuntimeConfig.bankAsset

class PaymentsPage extends Component {
  state = {
    isLoading: false,
    loadError: null,
    hasLoaded: false,
    data: [],
    nextPageToken: '',
  }

  loadTableData = async query => {
    this.setState({ isLoading: true, loadError: null })
    try {
      const res = await getPaymentsByAsset(BANK_ASSET)
      const payments = parsePaymentsApi(res?.data?.data || [])
      this.setState({ data: payments, nextPageToken: res?.data?.next?.id, isLoading: false, hasLoaded: true })
    } catch (e) {
      this.setState({ isLoading: false, loadError: e })
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
      nextPageToken,
    } = this.state
    const { router, windowWidth } = this.props
    return (
      <Page
        withSidebar
        withGlobalNav
        loadError={loadError}
        windowWidth={windowWidth}
      >
        <TablePayments
          payments={data || []}
          isLoading={!hasLoaded && isLoading}
          router={router}
          tableName={'Payments'}
          windowWidth={windowWidth}
          loadData={this.loadTableData}
          nextPageToken={nextPageToken}
          noLinkArrows
          noPagination
        />
      </Page>
    )
  }
}

PaymentsPage.propTypes = {
  query: PropTypes.object,
}

export default withRouter(PaymentsPage)
