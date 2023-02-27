/* eslint-disable camelcase */
import React from 'react'
import PropTypes from 'prop-types'
import IconBookOpen from 'assets/icons/icon-book-open'
import Image from 'next/image'
import iconRightArrowBlack from 'assets/icons/icon-arrow-right-black.svg'
import routes from 'routes'
import Table from './table'
import StatusIcon, {
  STATUS_ICON_TEXT_TO_ICON_TYPE_MAPPING,
} from 'components/status-icon'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

// capitalizes (uppercase first letter) of a string
export const capitalize = str => str.charAt(0).toUpperCase() + str.slice(1)

const makeColumns = ({ accounts, totalCount, tableName, isSmallViewport }) => {
  let columns = [
    {
      Header: 'Name',
      accessor: 'bank_reference.name',
    },
    {
      Header: 'Currency',
      accessor: 'bank_reference.currency',
    },
    {
      Header: 'Status',
      accessor: 'status',
      Cell: ({ cell: { row: { original: { status } } } }) => {
        return <StatusIcon type={STATUS_ICON_TEXT_TO_ICON_TYPE_MAPPING[capitalize(status)]} />
      },
    },
    {
      Header: '\u200b',
      accessor: 'id',
      style: { minWidth: 25, textAlign: 'right' },
      Cell: ({ cell: { row: { original: { id } } } }) => (
        <a href={`${routes.ADMIN_ACCOUNTS_PAGE}/${id}`}>
          <Image
            height={15}
            width={15}
            src={iconRightArrowBlack}
            alt={'View customer details'}
          />
        </a>
      ),
    },
  ]
  if (isSmallViewport) columns = [columns[0], columns[columns.length - 1]]
  return [
    {
      Header: tableName,
      titleIconComponent: <IconBookOpen color="#fff" />,
      columns: columns,
    },
  ]
}

const TableAccounts = ({
  accounts,
  isLoading,
  filters,
  loadData,
  pageIndex,
  pageSize,
  totalCount,
  tableName,
  loadError,
  nextPageToken,
  noPagination,
  withAccordion,
  windowWidth,
  headerTheme,
}) => {
  const isSmallViewport = windowWidth < 600
  const serverData = React.useMemo(() => accounts, [accounts])
  const columns = makeColumns({ accounts, totalCount, tableName, isSmallViewport })
  return (
    <Table
      columns={columns}
      data={serverData}
      isLoading={isLoading}
      totalCount={totalCount}
      loadData={loadData}
      pageIndex={pageIndex}
      pageSize={pageSize}
      loadError={loadError}
      nextPageToken={nextPageToken}
      noPagination={noPagination}
      withAccordion={withAccordion}
      headerTheme={headerTheme}
      isSmallViewport={isSmallViewport}
    />
  )
}

TableAccounts.propTypes = {
  accounts: PropTypes.arrayOf(PropTypes.shape({
    data: PropTypes.shape({
      name: PropTypes.string.isRequired,
      email: PropTypes.string.isRequired,
      phone_number: PropTypes.string.isRequired,
    }),
    id: PropTypes.number.isRequired,
  })),
  isLoading: PropTypes.bool,
  loadData: PropTypes.func,
  tableName: PropTypes.string,
  loadError: PropTypes.object,
  nextPageToken: PropTypes.shape({
    id: PropTypes.number,
    time: PropTypes.string,
  }),
  noPagination: PropTypes.bool,
  withAccordion: PropTypes.bool,
  windowWidth: PropTypes.number,
  headerTheme: PropTypes.string,
}

export default TableAccounts
