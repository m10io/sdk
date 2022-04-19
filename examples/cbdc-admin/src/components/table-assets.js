/* eslint-disable camelcase */
import { useMemo } from 'react'
import PropTypes from 'prop-types'
import IconDollarSign from '../assets/icons/icon-dollar-sign'
import iconRightArrowBlack from '../assets/icons/icon-arrow-right-black.svg'
import Table, {
  TABLE_HEADER_THEME_CARD,
} from './table'
import { Link } from 'react-router-dom'
import formatMoney from '../utils/format-money'
import { PRIMARY_COLOR } from '../consts'

const columns = ({ assets, totalCount, tableName, isSmallViewport }) => {
  let columns = [
    {
      Header: 'Name',
      accessor: 'name',
    },
    {
      Header: 'Account ID',
      accessor: 'accountId',
    },
    {
      Header: 'Issued',
      accessor: 'issued',
      Cell: ({ cell: { row: { original: { issued } } } }) => `$${formatMoney(issued)}`,
    },
    {
      Header: '\u200b',
      accessor: 'id',
      style: { width: 10 },
      Cell: ({ cell: { row: { original: { accountId } } } }) => (
        <Link to={`/accounts/${accountId}`}>
          <img
            style={{ width: 10 }}
            src={iconRightArrowBlack}
            alt={'View asset details'}
          />
        </Link>
      ),
    },
  ]
  if (isSmallViewport) columns = [columns[0], columns[columns.length - 1]]
  return [
    {
      Header: tableName,
      titleIconComponent: <IconDollarSign color={PRIMARY_COLOR} />,
      columns: columns,
    },
  ]
}

const TableAssets = ({
  assets,
  isLoading,
  filters,
  loadData,
  pageIndex,
  pageSize,
  totalCount,
  tableName = 'Assets',
  loadError,
  nextPageToken,
  noPagination,
  withAccordion,
  windowWidth,
  headerTheme=TABLE_HEADER_THEME_CARD,
}) => {
  const isSmallViewport = windowWidth < 600
  const serverData = useMemo(() => assets, [assets])
  const columnsImpl = columns({ assets, totalCount, tableName, isSmallViewport })
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

TableAssets.propTypes = {
  assets: PropTypes.arrayOf(PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    issued: PropTypes.number.isRequired,
    name: PropTypes.string.isRequired,
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

export default TableAssets
