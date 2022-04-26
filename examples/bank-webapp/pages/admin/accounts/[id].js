import React, { Component } from 'react'
import PropTypes from 'prop-types'
import Page from 'components/page'
import { withRouter } from 'next/router'
import TablePayments from 'components/table-payments'
import { AccountCard } from 'components/info-card'
import { TABLE_HEADER_THEME_CARD } from 'components/table'
import { parsePaymentsApi } from 'utils/api'
import {
  getAccountById,
} from 'lib/api/accounts'
import { getPaymentsByAccountIdAndAsset } from 'lib/api/payments'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const DEFAULT_ASSET = publicRuntimeConfig.bankAsset

class AccountPage extends Component {
  state = {
    // accounts
    isLoadingAccount: false,
    accountLoadError: null,
    account: {},

    // payments
    payments: [],
    isLoadingPayments: false,
    hasLoadedPayments: false,
    paymentsLoadError: null,
  }

  async getAccount() {
    try {
      this.setState({ isLoadingAccount: true, accountLoadError: null })
      const res = await getAccountById(this.props.id)
      const account = res?.data
      this.setState({ account, isLoadingAccount: false })
    } catch (e) {
      this.setState({ accountLoadError: e, isLoadingAccount: false })
    }
  }

  getPayments = async() => {
    this.setState({ isLoadingPayments: true, paymentsLoadError: null })
    try {
      const paymentsRes = await getPaymentsByAccountIdAndAsset(this.props.id, DEFAULT_ASSET)
      const payments = parsePaymentsApi(paymentsRes?.data?.data || [])
      this.setState({ payments, isLoadingPayments: false, hasLoadedPayments: true })
    } catch (e) {
      this.setState({ paymentsLoadError: e, isLoadingPayments: false })
    }
  }

  async componentDidMount() {
    await this.getAccount()
    await this.getPayments()
    this.timerId = setInterval(() => this.getPayments(), 3000)
  }

  componentWillUnmount() {
    clearInterval(this.timerId)
  }

  render() {
    const { windowWidth, id } = this.props
    const {
      isLoadingAccount,
      accountLoadError,
      account,
      isLoadingPayments,
      hasLoadedPayments,
      payments,
    } = this.state
    return (
      <Page
        withSidebar
        withGlobalNav
        loadError={accountLoadError}
        isLoading={isLoadingAccount}
        windowWidth={windowWidth}
      >
        <AccountCard account={account} />
        <TablePayments
          payments={payments}
          isLoading={!hasLoadedPayments && isLoadingPayments}
          noPagination
          headerTheme={TABLE_HEADER_THEME_CARD}
          windowWidth={windowWidth}
          accountId={id}
        />
      </Page>
    )
  }
}

AccountPage.getInitialProps = async ctx => {
  return { id: ctx.query.id }
}

AccountPage.propTypes = {
  id: PropTypes.string.isRequired,
  windowWidth: PropTypes.number,
}

export default withRouter(AccountPage)
