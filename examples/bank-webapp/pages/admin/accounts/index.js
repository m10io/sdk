/* eslint-disable camelcase */
import React, { Component } from 'react'
import PropTypes from 'prop-types'
import Page from 'components/page'
import { withRouter } from 'next/router'
import TableAccounts from 'components/table-accounts'
import { getAccounts } from 'lib/api/accounts'

class AccountsPage extends Component {
  state = {
    isLoading: false,
    loadError: null,
    data: [],
  }

  loadTableData = async query => {
    this.setState({ isLoading: true, loadError: null })
    try {
      const res = await getAccounts(query)
      this.setState({ data: res?.data, isLoading: false })
    } catch (e) {
      this.setState({ isLoading: false, loadError: e })
    }
  }

  async componentDidMount() {
    await this.loadTableData()
  }

  render() {
    const {
      isLoading,
      loadError,
      data,
    } = this.state
    const { router, query, windowWidth } = this.props
    return (
      <Page
        withSidebar
        withGlobalNav
        loadError={loadError}
        windowWidth={windowWidth}
      >
        <TableAccounts
          accounts={data?.data || []}
          isLoading={isLoading}
          filters={query}
          router={router}
          loadData={this.loadTableData}
          nextPageToken={data?.next_page_token || {}}
          tableName={'Accounts'}
          windowWidth={windowWidth}
          noPagination
        />
      </Page>
    )
  }
}

AccountsPage.propTypes = {
  query: PropTypes.object,
}

export default withRouter(AccountsPage)
