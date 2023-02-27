/* eslint-disable camelcase */
import React, { Component } from 'react'
import moment from 'moment'
import PropTypes from 'prop-types'
import Page from 'components/page'
import Image from 'next/image'
import { withRouter } from 'next/router'
import Table from 'components/table'
import IconBookOpen from 'assets/icons/icon-book-open'
import iconRightArrowBlack from 'assets/icons/icon-arrow-right-black.svg'
import axios from 'axios'
import routes from 'routes'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

// Wallets Table
const makeColumns = ({ wallets, isSmallViewport }) => {
  const columns = [
    {
      Header: 'Name',
      accessor: 'display_name',
    },
    {
      Header: 'Currency',
      accessor: 'currency',
    },
    {
      Header: 'Created At',
      accessor: 'created_at',
      Cell: ({ cell: { row: { original: { created_at } } } }) => (
        moment(created_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
      ),
    },
    {
      Header: '\u200b',
      accessor: 'id',
      style: { minWidth: 25, textAlign: 'right' },
      Cell: ({ cell: { row: { original: { id } } } }) => (
        <a href={`${routes.OFFLINE_PAYMENTS_WALLETS_PAGE}/${id}`}>
          <Image
            height={15}
            width={15}
            src={iconRightArrowBlack}
            alt={'View wallet payments details'}
          />
        </a>
      ),
    },
  ]
  return [
    {
      Header: 'Offline Payment Wallets',
      titleIconComponent: <IconBookOpen color="#fff" />,
      columns: columns,
    },
  ]
}

const TableWallets = ({
  wallets,
  isLoading,
  loadData,
  tableName,
  loadError,
  windowWidth,
}) => {
  const isSmallViewport = windowWidth < 600
  const serverData = React.useMemo(() => wallets, [wallets])
  const columns = makeColumns({ wallets, tableName, isSmallViewport })
  return (
    <Table
      columns={columns}
      data={serverData}
      isLoading={isLoading}
      loadData={loadData}
      loadError={loadError}
      isSmallViewport={isSmallViewport}
      noPagination
    />
  )
}

class BursarWalletsPage extends Component {
  state = {
    isLoading: false,
    loadError: null,
    hasLoaded: false,
    data: [],
  }

  loadTableData = async query => {
    this.setState({ isLoading: true, loadError: null })
    try {
      const res = await axios.get(routes.OFFLINE_PAYMENTS_WALLETS_API)
      const wallets = res?.data?.data
      this.setState({ data: wallets, isLoading: false, hasLoaded: true })
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
        withSidebar
        loadError={loadError}
        windowWidth={windowWidth}
      >
        <TableWallets
          wallets={data}
          isLoading={!hasLoaded && isLoading}
          router={router}
          windowWidth={windowWidth}
          loadData={this.loadTableData}
        />
      </Page>
    )
  }
}

BursarWalletsPage.propTypes = {
  query: PropTypes.object,
}

export default withRouter(BursarWalletsPage)
