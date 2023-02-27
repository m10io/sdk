/* eslint-disable camelcase */
import React, { useState, useEffect } from 'react'
import Image from 'next/image'
import moment from 'moment'
import StatusIcon, {
  STATUS_ICON_TYPE_PENDING,
  STATUS_ICON_TYPE_FRAUD,
} from 'components/status-icon'
import Table from 'components/table'
import Link, { LINK_THEME_BLUE } from 'components/link'
import lodashSortBy from 'lodash/sortBy'
import ToggleSwitch from 'components/toggle-switch'
import IconDollarSign from 'icons/icon-dollar-sign'
import iconRightArrowBlack from 'assets/icons/icon-arrow-right-black.svg'
import formatMoney from 'utils/format-money'
import routes from 'routes'

// Offline Payments Table
const makeColumns = ({ payments, tableName, isSmallViewport }) => {
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
          ? debited_at === 'FRAUD'
            ? <StatusIcon type={STATUS_ICON_TYPE_FRAUD} />
            : moment(debited_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
          : <StatusIcon type={STATUS_ICON_TYPE_PENDING} />
      ),
    },
    {
      Header: 'Credited',
      accessor: 'credited_at',
      Cell: ({ cell: { row: { original: { credited_at } } } }) => (
        credited_at
          ? credited_at === 'FRAUD'
            ? <StatusIcon type={STATUS_ICON_TYPE_FRAUD} />
            : moment(credited_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
          : <StatusIcon type={STATUS_ICON_TYPE_PENDING} />
      ),
    },
    {
      Header: '\u200b',
      accessor: 'id',
      style: { minWidth: 15 },
      Cell: ({ cell: { row: { original: { id, credited_at } } } }) => {
        const isFraud = credited_at === 'FRAUD'
        const url = `${routes.OFFLINE_PAYMENTS_PAYMENTS_PAGE}/${id}`
        return (isFraud
          ? null
          : (
            <a href={url}>
              <Image
                height={15}
                width={15}
                src={iconRightArrowBlack}
                alt={'View offline payment details'}
              />
            </a>
          )
        )
      }
    }
  ]
  return [
    {
      Header: tableName,
      titleIconComponent: <IconDollarSign color={'#fff'} />,
      columns: columns,
    },
  ]
}

const TableOfflinePayments = ({
  payments = [],
  fraudPayments = [],
  isLoading,
  loadData,
  tableName,
  loadError,
  windowWidth,
  limit,
  includeFraudPaymentsToggle,
}) => {
  const isSmallViewport = windowWidth < 600
  const [showFraudPayments, setShowFraudPayments] = useState(false)

  const initialPayments = React.useMemo(() => payments, [payments])
  const [serverData, setServerData] = useState(initialPayments)
  useEffect(() => {
    setServerData(initialPayments)
    if (showFraudPayments) {
      const paymentsWithFraudPayments = payments.concat(
        fraudPayments.map(p => ({ ...p, debited_at: 'FRAUD', credited_at: 'FRAUD', sortDate: moment().valueOf(p.uploaded_at) }))
      )
      const paymentsWithFraudPaymentsSortedByDate = lodashSortBy(
        paymentsWithFraudPayments,
        p => p.sortDate
      )
      setServerData(paymentsWithFraudPaymentsSortedByDate)
    } else {
      setServerData(initialPayments)
    }
  }, [showFraudPayments, payments])

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
      rightHeaderContent={includeFraudPaymentsToggle
        ? (
          <div>
            <ToggleSwitch
              label={'Fraud Payments'}
              onChange={() => {
                setShowFraudPayments(!showFraudPayments)
              }}
              checked={!!showFraudPayments}
            />
          </div>
        ) : null
      }
    />
  )
}

export default TableOfflinePayments
