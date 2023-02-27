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
    isLoadingCbdcPayments: false,
    hasLoadedCbdcPayments: false,
    isLoadingDrmPayments: false,
    hasLoadedDrmPayments: false,
    cbdcPaymentsLoadError: null,
    cbdcPayments: [],
    drmPaymentsLoadError: null,
    drmPayments: [],
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

  getCbdcPayments = async() => {
    this.setState({ isLoadingCbdcPayments: true, cbdcPaymentsLoadError: null })
    try {
      const paymentsRes = await getPaymentsByAccountIdAndAsset(this.props.id, DEFAULT_ASSET, 'indirect_cbdc')
      const payments = parsePaymentsApi(paymentsRes?.data?.data || [])
      this.setState({ cbdcPayments: payments, isLoadingCbdcPayments: false, hasLoadedCbdcPayments: true })
    } catch (e) {
      this.setState({ cbdcPaymentsLoadError: e, isLoadingCbdcPayments: false })
    }
  }

  getDrmPayments = async() => {
    this.setState({ isLoadingDrmPayments: true, drmPaymentsLoadError: null })
    try {
      const paymentsRes = await getPaymentsByAccountIdAndAsset(this.props.id, DEFAULT_ASSET, 'regulated')
      const payments = parsePaymentsApi(paymentsRes?.data?.data || [])
      this.setState({ drmPayments: payments, isLoadingDrmPayments: false, hasLoadedDrmPayments: true })
    } catch (e) {
      this.setState({ drmPaymentsLoadError: e, isLoadingDrmPayments: false })
    }
  }

  async componentDidMount() {
    await this.getAccount()
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
      isLoadingAccount,
      accountLoadError,
      account,
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
        loadError={accountLoadError}
        isLoading={isLoadingAccount}
        windowWidth={windowWidth}
      >
        <AccountCard account={account} />
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

AccountPage.getInitialProps = async ctx => {
  return { id: ctx.query.id }
}

AccountPage.propTypes = {
  id: PropTypes.string.isRequired,
  windowWidth: PropTypes.number,
}

export default withRouter(AccountPage)
