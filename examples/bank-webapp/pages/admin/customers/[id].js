import React, { Component } from 'react'
import PropTypes from 'prop-types'
import Page from 'components/page'
import {
  CustomerInfoCard,
} from 'components/info-card'
import { withRouter } from 'next/router'
import TablePayments from 'components/table-payments'
import { TABLE_HEADER_THEME_CARD } from 'components/table'
import { parsePaymentsApi } from 'utils/api'
import {
  getCustomerById,
} from 'lib/api/customers'
import { getPaymentsByAccountIdAndAsset } from 'lib/api/payments'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const DEFAULT_ASSET = publicRuntimeConfig.bankAsset

class CustomerPage extends Component {
  state = {
    // customer
    isLoadingCustomer: false,
    customerLoadError: null,
    customer: {},

    // payments
    isLoadingCbdcPayments: false,
    hasLoadedCbdcPayments: false,
    isLoadingDrmPayments: false,
    hasLoadedDrmPayments: false,
    cbdcPaymentsLoadError: null,
    cbdcPayments: [],
    drmPaymentsLoadError: null,
    drmPayments: [],
  }

  async getCustomer() {
    try {
      this.setState({ isLoadingCustomer: true, customerLoadError: null })
      const res = await getCustomerById(this.props.id)
      const customer = res?.data
      this.setState({ customer, isLoadingCustomer: false })
    } catch (e) {
      this.setState({ customerLoadError: e, isLoadingCustomer: false })
    }
  }

  getCbdcPayments = async() => {
    try {
      this.setState({ isLoadingCbdcPayments: true, cbdcPaymentsLoadError: null })
      const customer = this.state.customer
      const paymentsRes = await getPaymentsByAccountIdAndAsset(customer?.account_id, DEFAULT_ASSET, 'indirect_cbdc')
      const payments = parsePaymentsApi(paymentsRes?.data?.data || [])
      this.setState({ cbdcPayments: payments, isLoadingCbdcPayments: false, hasLoadedCbdcPayments: true })
    } catch (e) {
      this.setState({ cbdcPaymentsLoadError: e, isLoadingCbdcPayments: false })
    }
  }

  getDrmPayments = async() => {
    try {
      this.setState({ isLoadingDrmPayments: true, drmPaymentsLoadError: null })
      const customer = this.state.customer
      const paymentsRes = await getPaymentsByAccountIdAndAsset(customer?.account_id, DEFAULT_ASSET, 'regulated')
      const payments = parsePaymentsApi(paymentsRes?.data?.data || [])
      this.setState({ drmPayments: payments, isLoadingDrmPayments: false, hasLoadedDrmPayments: true })
    } catch (e) {
      this.setState({ drmPaymentsLoadError: e, isLoadingDrmPayments: false })
    }
  }

  async componentDidMount() {
    await this.getCustomer()
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
    const { windowWidth, id } = this.props
    const {
      isLoadingCustomer,
      customerLoadError,
      customer,
      isLoadingCbdcPayments,
      hasLoadedCbdcPayments,
      cbdcPayments,
      isLoadingDrmPayments,
      hasLoadedDrmPayments,
      drmPayments,
    } = this.state
    return (
      <Page
        withSidebar
        withGlobalNav
        loadError={customerLoadError}
        isLoading={isLoadingCustomer}
        windowWidth={windowWidth}
      >
        <CustomerInfoCard customer={customer} />
        <TablePayments
          payments={cbdcPayments}
          isLoading={!hasLoadedCbdcPayments && isLoadingCbdcPayments}
          tableName={'CBDC Payments'}
          noPagination
          headerTheme={TABLE_HEADER_THEME_CARD}
          windowWidth={windowWidth}
          accountId={id}
        />
        <TablePayments
          payments={drmPayments}
          isLoading={!hasLoadedDrmPayments && isLoadingDrmPayments}
          tableName={'DRM Payments'}
          noPagination
          headerTheme={TABLE_HEADER_THEME_CARD}
          windowWidth={windowWidth}
          accountId={id}
        />
      </Page>
    )
  }
}

CustomerPage.getInitialProps = async ctx => {
  return { id: ctx.query.id }
}

CustomerPage.propTypes = {
  id: PropTypes.string.isRequired,
  windowWidth: PropTypes.number,
}

export default withRouter(CustomerPage)
