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
    isLoadingCbdcPayments: false,
    hasLoadedCbdcPayments: false,
    isLoadingDrmPayments: false,
    hasLoadedDrmPayments: false,
    cbdcPaymentsLoadError: null,
    cbdcPayments: [],
    drmPaymentsLoadError: null,
    drmPayments: [],
    nextCbdcPageToken: '',
    nextDrmPageToken: '',
  }

  getCbdcPayments = async() => {
    const { hasLoadedCbdcPayments, cbdcPaymentsLoadError } = this.state
    this.setState({
      isLoadingCbdcPayments: true,
      cbdcPaymentsLoadError: (hasLoadedCbdcPayments && cbdcPaymentsLoadError) ?? null,
    })
    try {
      const res = await getPaymentsByAsset(BANK_ASSET, 'indirect_cbdc')
      const payments = parsePaymentsApi(res?.data?.data || [])
      this.setState({
        cbdcPayments: payments,
        nextCbdcPageToken: res?.data?.next?.id,
        isLoadingCbdcPayments: false,
        hasLoadedCbdcPayments: true,
        cbdcPaymentsLoadError: null,
      })
    } catch (e) {
      this.setState({
        cbdcPaymentsLoadError: e,
        isLoadingCbdcPayments: false,
        hasLoadedCbdcPayments: true,
      })
    }
  }

  getDrmPayments = async() => {
    const { hasLoadedDrmPayments, drmPaymentsLoadError } = this.state
    this.setState({
      isLoadingDrmPayments: true,
      drmPaymentsLoadError: (hasLoadedDrmPayments && drmPaymentsLoadError) ?? null,
    })
    try {
      const res = await getPaymentsByAsset(BANK_ASSET, 'regulated')
      const payments = parsePaymentsApi(res?.data?.data || [])
      this.setState({
        drmPayments: payments,
        nextDrmPageToken: res?.data?.next?.id,
        isLoadingDrmPayments: false,
        hasLoadedDrmPayments: true,
        drmPaymentsLoadError: null,
      })
    } catch (e) {
      this.setState({
        drmPaymentsLoadError: e,
        isLoadingDrmPayments: false,
        hasLoadedDrmPayments: true,
      })
    }
  }

  async componentDidMount() {
    await this.getCbdcPayments()
    await this.getDrmPayments()
    this.timerId = setInterval(() => {
      this.getCbdcPayments()
      this.getDrmPayments()
    }, 3000)
  }

  componentWillUnmount() {
    clearInterval(this.timerId)
  }

  render() {
    const {
      isLoadingCbdcPayments,
      hasLoadedCbdcPayments,
      cbdcPayments,
      cbdcPaymentsLoadError,
      isLoadingDrmPayments,
      hasLoadedDrmPayments,
      drmPayments,
      drmPaymentsLoadError,
      nextCbdcPageToken,
      nextDrmPageToken,
    } = this.state
    const { router, windowWidth } = this.props
    return (
      <Page
        withSidebar
        withGlobalNav
        loadError={cbdcPaymentsLoadError || drmPaymentsLoadError}
        windowWidth={windowWidth}
      >
        <TablePayments
          payments={cbdcPayments || []}
          isLoading={!hasLoadedCbdcPayments && isLoadingCbdcPayments}
          router={router}
          tableName={'CBDC Payments'}
          windowWidth={windowWidth}
          loadData={this.getCbdcPayments}
          nextPageToken={nextCbdcPageToken}
          noLinkArrows
          noPagination
          limit={50}
        />
        <TablePayments
          payments={drmPayments || []}
          isLoading={!hasLoadedDrmPayments && isLoadingDrmPayments}
          router={router}
          tableName={'DRM Payments'}
          windowWidth={windowWidth}
          loadData={this.getDrmPayments}
          nextPageToken={nextDrmPageToken}
          noLinkArrows
          noPagination
          limit={50}
        />
      </Page>
    )
  }
}

PaymentsPage.propTypes = {
  query: PropTypes.object,
}

export default withRouter(PaymentsPage)
