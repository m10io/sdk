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
    isLoadingPayments: false,
    hasLoadedPayments: false,
    paymentsLoadError: null,
    payments: [],
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

  getPayments = async() => {
    try {
      this.setState({ isLoadingPayments: true, paymentsLoadError: null })
      const paymentsRes = await getPaymentsByAccountIdAndAsset(this.props.id, DEFAULT_ASSET)
      const payments = parsePaymentsApi(paymentsRes?.data?.data || [])
      this.setState({ payments: payments, isLoadingPayments: false, hasLoadedPayments: true })
    } catch (e) {
      this.setState({ paymentsLoadError: e, isLoadingPayments: false })
    }
  }

  async componentDidMount() {
    await this.getCustomer()
    await this.getPayments()
    this.timerId = setInterval(() => this.getPayments(), 3000)
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
      isLoadingPayments,
      hasLoadedPayments,
      payments,
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
          payments={payments}
          isLoading={!hasLoadedPayments && isLoadingPayments}
          tableName={'Payments'}
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
