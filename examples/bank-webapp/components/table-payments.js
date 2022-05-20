/* eslint-disable camelcase, max-len */
import React from 'react'
import classnames from 'classnames'
import moment from 'moment'
import PropTypes from 'prop-types'
import Image from 'next/image'
import Table from './table'
import Link, {
  LINK_THEME_BLUE,
} from 'components/link'
import StatusIcon, {
  STATUS_ICON_TYPE_WITHDRAWAL,
  STATUS_ICON_TYPE_DEPOSIT,
} from 'components/status-icon'
import IconDollarSign from 'assets/icons/icon-dollar-sign'
import iconRightArrowBlack from 'assets/icons/icon-arrow-right-black.svg'
import iconArrowRightGray from 'assets/icons/icon-arrow-right-gray.svg'
import routes from 'routes'
import formatDollarValue from 'utils/format-money'
import styles from 'styles/table.module.scss'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const BANK_ASSET = publicRuntimeConfig.bankAsset
moment.suppressDeprecationWarnings = true

const renderNameLink = ({ accountType, name, id }) => {
  const linkTo = accountType === 'Customer' && routes.ADMIN_CUSTOMERS_PAGE
  return linkTo
    ? <Link theme={LINK_THEME_BLUE} href={linkTo && `${linkTo}/${id}`}>{name}</Link>
    : name
}

const renderTableReceiverName = ({ metadataType, receiverName }) => {
  const isUnknownReceiver = receiverName === 'Unknown' || !receiverName
  return (
    metadataType === 'withdraw' && isUnknownReceiver
      ? <StatusIcon type={STATUS_ICON_TYPE_WITHDRAWAL} />
      : metadataType === 'deposit' && isUnknownReceiver
        ? <StatusIcon type={STATUS_ICON_TYPE_DEPOSIT} />
        : receiverName
  )
}

const makeColumns = ({ payments, totalCount, tableName, isSmallViewport, noLinkArrows, accountId, customBaseUrl }) => {
  let columns = [
    {
      Header: 'Sender',
      Cell: ({ cell: { row: { getToggleRowExpandedProps, isExpanded, original: { senderName, senderAccountType, senderId, steps, ...etc } } } }) => {
        const isSteps = Array.isArray(steps) && steps?.length
        const toggleRowExpandedProps = isSteps ? getToggleRowExpandedProps() : {}
        return (
          <div className={styles.cellTextWithIcon}>
            <div
              className={classnames(
                isSteps && styles.cellIconPreText,
                isExpanded && styles.cellIconPreTextExpanded,
              )}
              {...toggleRowExpandedProps}
            >
              {isSteps && (
                <Image
                  height={15}
                  width={15}
                  src={iconArrowRightGray}
                  alt={'Steps'}
                />
              )}
            </div>
            {renderNameLink({ accountType: senderAccountType, name: senderName, id: senderId })}
          </div>
        )
      },
    },
    {
      Header: 'Receiver',
      Cell: ({ cell: { row: { original: { amount, receiverName, receiverId, receiverAccountType, paymentType, steps, ...etc } } } }) => {
        steps = Array.isArray(steps) ? steps : []
        const name = renderNameLink({ accountType: receiverAccountType, name: receiverName, id: receiverId })
        const finalMetaDataType = ((steps[steps.length - 1] || {}).metadata || [])[0]?.metadata_type
        const renderedName = renderTableReceiverName({ metadataType: finalMetaDataType, receiverName: name })
        return renderedName
      },
    },
    {
      Header: 'Amount',
      Cell: ({ cell: { row: { original: { amount } } } }) => `$${formatDollarValue(amount)}`,
    },
    {
      Header: 'Date/Time',
      // 2022-05-10 19:33:18.419719
      Cell: ({ cell: { row: { original: { timestamp } } } }) => moment(timestamp, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a'),
    },
    ...(!noLinkArrows ? [
      {
        Header: '\u200b',
        accessor: 'id',
        style: { minWidth: 15 },
        Cell: ({ cell: { row: { original: { id } } } }) => {
          const url = customBaseUrl
            ? `${customBaseUrl}/${id}`
            : `${routes.ADMIN_PAYMENTS_PAGE}/${id}?asset=${BANK_ASSET}&accountId=${accountId}`
          return (
            <a href={url}>
              <Image
                height={15}
                width={15}
                src={iconRightArrowBlack}
                alt={'View payment details'}
              />
            </a>
          )
        }
      }
    ] : []),
  ]
  if (isSmallViewport) columns = [columns[0], columns[1], columns[columns.length - 1]]
  return [
    {
      Header: tableName || 'Payments',
      titleIconComponent: <IconDollarSign color={publicRuntimeConfig.bankPrimaryColor} />,
      columns: columns
    },
  ]
}

const SubRowTable = ({
  isLoading,
  row,
  isSmallViewport,
}) => {
  const transfers = row?.original?.steps || []
  return transfers.map((tx, index) => {
    const timeFormatted = moment(tx.timestamp, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
    const metadataType = (Array.isArray(tx.metadata) ? tx.metadata[0] : {})?.metadata_type
    return (
      <tr key={index} className={styles.expandedTxRow}>
        <td className={styles.expandedTxRowTd}>
          {tx.sender?.name ?? 'Unknown'}
        </td>
        <td className={styles.expandedTxRowTd}>
          {renderTableReceiverName({ metadataType, receiverName: tx?.receiver?.name }) ?? 'Unknown'}
        </td>
        {!isSmallViewport && (
          <td className={styles.expandedTxRowTd}>
            {`$${formatDollarValue(tx.amount)}`}
          </td>
        )}
        {!isSmallViewport && (
          <td className={styles.expandedTxRowTd}>
            <div>{timeFormatted}</div>
          </td>
        )}
        <td className={styles.expandedTxRowTd}>
        </td>
      </tr>
    )
  })
}

const TablePayments = ({
  payments,
  isLoading,
  loadData,
  totalCount,
  tableName,
  nextPageToken,
  noPagination,
  withAccordion,
  windowWidth,
  headerTheme,
  noLinkArrows,
  accountId,
  onlyForwardPagination,
  customBaseUrl,
  limit,
}) => {
  const isSmallViewport = windowWidth < 600
  const serverData = React.useMemo(() => payments, [payments])
  const columns = makeColumns({ payments, totalCount, tableName, isSmallViewport, noLinkArrows, accountId, customBaseUrl })
  return (
    <Table
      columns={columns}
      data={serverData}
      isLoading={isLoading}
      totalCount={totalCount}
      loadData={loadData}
      nextPageToken={nextPageToken}
      noPagination={noPagination}
      withAccordion={withAccordion}
      headerTheme={headerTheme}
      isSmallViewport={isSmallViewport}
      renderRowSubComponent={SubRowTable}
      onlyForwardPagination={onlyForwardPagination}
      limit={limit}
    />
  )
}

TablePayments.propTypes = {
  payments: PropTypes.arrayOf(PropTypes.object),
  isLoading: PropTypes.bool,
  loadData: PropTypes.func,
  tableName: PropTypes.string,
  noPagination: PropTypes.bool,
  withAccordion: PropTypes.bool,
  windowWidth: PropTypes.number,
  headerTheme: PropTypes.string,
  accountId: PropTypes.string,
  onlyForwardPagination: PropTypes.bool,
  customBaseUrl: PropTypes.string,
}

export default TablePayments
