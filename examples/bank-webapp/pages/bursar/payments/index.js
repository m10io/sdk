/* eslint-disable camelcase */
import React, { Component } from 'react'
import PropTypes from 'prop-types'
import Page from 'components/page'
import { withRouter } from 'next/router'
import TablePayments from 'components/table-payments'
// import axios from 'axios'

const MOCK_PAYMENTS = [
  {
    id: '1',
    senderName: 'Sender Name',
    senderCustomerId: '122',
    receiverName: 'Receiver Name',
    receiverCustomerId: '123',
    timestamp: '2022-05-03T20:23:40.277Z',
    amount: '10000',
    senderInstrument: 'USD',
    txChain: [],
  },
]

class BursarPaymentsPage extends Component {
  state = {
    isLoading: false,
    loadError: null,
    hasLoaded: false,
    data: [],
  }

  loadTableData = async query => {
    this.setState({ isLoading: true, loadError: null })
    try {
      // TODO: update api url
      // const res = await axios.get('')
      this.setState({ data: MOCK_PAYMENTS, isLoading: false, hasLoaded: true })
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
    } = this.state
    const { router, windowWidth } = this.props
    return (
      <Page
        withGlobalNav
        loadError={loadError}
        windowWidth={windowWidth}
      >
        <TablePayments
          payments={data || []}
          isLoading={!hasLoaded && isLoading}
          router={router}
          tableName={'Bursar Payments'}
          windowWidth={windowWidth}
          loadData={this.loadTableData}
          noPagination
          customBaseUrl={'/bursar/payments'}
        />
      </Page>
    )
  }
}

BursarPaymentsPage.propTypes = {
  query: PropTypes.object,
}

export default withRouter(BursarPaymentsPage)
