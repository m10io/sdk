/* eslint-disable camelcase */
import React, { Component } from 'react'
import PropTypes from 'prop-types'
import Page from 'components/page'
import Image from 'next/image'
import { withRouter } from 'next/router'
import Table from 'components/table'
import IconBookOpen from 'assets/icons/icon-book-open'
import iconRightArrowBlack from 'assets/icons/icon-arrow-right-black.svg'
// import axios from 'axios'
// import routes from 'routes'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

// Wallets Table
const makeColumns = ({ wallets, isSmallViewport }) => {
  const columns = [
    {
      Header: 'Name',
      accessor: 'name',
    },
    {
      Header: '\u200b',
      accessor: 'id',
      style: { minWidth: 25, textAlign: 'right' },
      // TODO: update route url
      Cell: ({ cell: { row: { original: { id } } } }) => (
        <a href={'/bursar/payments'}>
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
      Header: 'Bursar Wallets',
      titleIconComponent: <IconBookOpen color={publicRuntimeConfig.bankPrimaryColor} />,
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

const MOCK_WALLETS = [
  {
    name: 'Wallet 1',
    id: '1',
  },
  {
    name: 'Wallet 2',
    id: '2',
  }
]

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
      // TODO: update api url
      // const res = await axios.get('')
      const wallets = MOCK_WALLETS
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
