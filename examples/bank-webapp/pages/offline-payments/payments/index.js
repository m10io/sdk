/* eslint-disable camelcase */
import React, { Component } from 'react'
import axios from 'axios'
import PropTypes from 'prop-types'
import moment from 'moment'
import Image from 'next/image'
import Page from 'components/page'
import Link, { LINK_THEME_BLUE } from 'components/link'
import Table from 'components/table'
import StatusIcon, { STATUS_ICON_TYPE_PENDING } from 'components/status-icon'
import IconDollarSign from 'icons/icon-dollar-sign'
import iconRightArrowBlack from 'assets/icons/icon-arrow-right-black.svg'
import routes from 'routes'
import formatMoney from 'utils/format-money'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const CURRENCY = publicRuntimeConfig.bankAsset

// Offline Payments Table
const makeColumns = ({ payments, isSmallViewport }) => {
  const columns = [
    {
      Header: 'Sender',
      accessor: 'from_display_name',
      Cell: ({ cell: { row: { original: { from_display_name, from_wallet } } } }) => (
        <Link theme={LINK_THEME_BLUE} href={`${routes.OFFLINE_PAYMENTS_WALLETS_PAGE}/${from_wallet}`}>
          {from_display_name}
        </Link>
      ),
    },
    {
      Header: 'Receiver',
      accessor: 'to_display_name',
      Cell: ({ cell: { row: { original: { to_display_name, to_wallet } } } }) => (
        <Link theme={LINK_THEME_BLUE} href={`${routes.OFFLINE_PAYMENTS_WALLETS_PAGE}/${to_wallet}`}>
          {to_display_name}
        </Link>
      ),
    },
    {
      Header: 'Amount',
      accessor: 'amount',
      Cell: ({ cell: { row: { original: { amount, currency } } } }) => (
        `${formatMoney(amount)} ${currency}`
      ),
    },
    {
      Header: 'Uploaded',
      accessor: 'uploaded_at',
      Cell: ({ cell: { row: { original: { uploaded_at } } } }) => (
        moment(uploaded_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
      ),
    },
    {
      Header: 'Debited',
      accessor: 'debited_at',
      Cell: ({ cell: { row: { original: { debited_at } } } }) => (
        debited_at
          ? moment(debited_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
          : <StatusIcon type={STATUS_ICON_TYPE_PENDING} />
      ),
    },
    {
      Header: 'Credited',
      accessor: 'credited_at',
      Cell: ({ cell: { row: { original: { credited_at } } } }) => (
        credited_at
          ? moment(credited_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
          : <StatusIcon type={STATUS_ICON_TYPE_PENDING} />
      ),
    },
    {
      Header: '\u200b',
      accessor: 'id',
      style: { minWidth: 15 },
      Cell: ({ cell: { row: { original: { id } } } }) => {
        const url = `${routes.OFFLINE_PAYMENTS_PAYMENTS_PAGE}/${id}`
        return (
          <a href={url}>
            <Image
              height={15}
              width={15}
              src={iconRightArrowBlack}
              alt={'View offline payment details'}
            />
          </a>
        )
      }
    }
  ]
  return [
    {
      Header: 'Offline Payments',
      titleIconComponent: <IconDollarSign color={'#000000'} />,
      columns: columns,
    },
  ]
}

export const TableOfflinePayments = ({
  payments,
  isLoading,
  loadData,
  tableName,
  loadError,
  windowWidth,
  limit,
}) => {
  const isSmallViewport = windowWidth < 600
  const serverData = React.useMemo(() => payments, [payments])
  const columns = makeColumns({ payments, tableName, isSmallViewport })
  return (
    <Table
      columns={columns}
      data={serverData}
      isLoading={isLoading}
      loadData={loadData}
      loadError={loadError}
      isSmallViewport={isSmallViewport}
      noPagination
      limit={limit}
    />
  )
}

class OfflinePaymentsPage extends Component {
  state = {
    isLoading: false,
    loadError: null,
    hasLoaded: false,
    data: [],
  }

  loadTableData = async query => {
    this.setState({ isLoading: true, loadError: null })
    try {
      const res = await axios.get(`${routes.OFFLINE_PAYMENTS_PAYMENTS_API}?currency=${CURRENCY}`)
      this.setState({ data: res?.data?.data, isLoading: false, hasLoaded: true })
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
    const { windowWidth } = this.props
    return (
      <Page
        withGlobalNav
        withSidebar
        loadError={loadError}
        windowWidth={windowWidth}
        navLogoRoute={routes.OFFLINE_PAYMENTS_PAYMENTS_PAGE}
      >
        <TableOfflinePayments
          payments={data || []}
          isLoading={!hasLoaded && isLoading}
          tableName={'Offline Payments'}
          windowWidth={windowWidth}
          loadData={this.loadTableData}
          noPagination
          limit={50}
        />
      </Page>
    )
  }
}

OfflinePaymentsPage.propTypes = {
  query: PropTypes.object,
}

export default OfflinePaymentsPage
