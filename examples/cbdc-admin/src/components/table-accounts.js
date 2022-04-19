/* eslint-disable camelcase */
import React from 'react'
import PropTypes from 'prop-types'
import IconBookOpen from '../assets/icons/icon-book-open'
import iconRightArrowBlack from '../assets/icons/icon-arrow-right-black.svg'
import { Link } from 'react-router-dom'
import Table, {
  TABLE_HEADER_THEME_CARD,
} from './table'
import formatMoney from '../utils/format-money'
import { PRIMARY_COLOR } from '../consts'
import routes from '../routes'

const columns = ({ accounts, totalCount, tableName, isSmallViewport }) => {
  let columns = [
    {
      Header: 'ID',
      accessor: 'accountId',
    },
    {
      Header: 'Name',
      accessor: 'name',
    },
    {
      Header: 'Asset',
      accessor: 'code',
    },
    {
      Header: 'Issued',
      Cell: ({ cell: { row: { original: { issued }}}}) => formatMoney(issued),
    },
    {
      Header: '\u200b',
      accessor: '',
      style: { width: 10 },
      Cell: ({ cell: { row: { original: { accountId } } } }) => (
        <Link to={`${routes.ACCOUNTS_PAGE_ROUTE}/${accountId}`}>
          <img
            style={{ width: 10 }}
            src={iconRightArrowBlack}
            alt={'View account details'}
          />
        </Link>
      ),
    },
  ]
  if (isSmallViewport) columns = [columns[0], columns[columns.length - 1]]
  return [
    {
      Header: tableName,
      titleIconComponent: <IconBookOpen color={PRIMARY_COLOR} />,
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
  headerTheme=TABLE_HEADER_THEME_CARD,
}) => {
  const isSmallViewport = windowWidth < 600
  const serverData = React.useMemo(() => accounts, [accounts])
  const columnsImpl = columns({ accounts, totalCount, tableName, isSmallViewport })
  return (
    <Table
      columns={columnsImpl}
      data={serverData}
      isLoading={isLoading}
      totalCount={totalCount}
      loadData={loadData}
      pageIndex={pageIndex}
      pageSize={pageSize}
      loadError={loadError}
      noPagination={noPagination}
      withAccordion={withAccordion}
      headerTheme={headerTheme}
      isSmallViewport={isSmallViewport}
    />
  )
}

TableAccounts.propTypes = {
  accounts: PropTypes.arrayOf(PropTypes.shape({
    code: PropTypes.string.isRequired,
    name: PropTypes.string.isRequired,
    accountId: PropTypes.string.isRequired,
    decimals: PropTypes.number.isRequired,
    issued: PropTypes.number.isRequired,
  })),
  isLoading: PropTypes.bool,
  loadData: PropTypes.func,
  tableName: PropTypes.string,
  loadError: PropTypes.object,
  noPagination: PropTypes.bool,
  withAccordion: PropTypes.bool,
  windowWidth: PropTypes.number,
  headerTheme: PropTypes.string,
}

export default TableAccounts
