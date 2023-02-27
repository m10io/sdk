/* eslint-disable camelcase */
import React from 'react'
import PropTypes from 'prop-types'
import Image from 'next/image'
import IconUsers from 'assets/icons/icon-users'
import iconRightArrowBlack from 'assets/icons/icon-arrow-right-black.svg'
import routes from 'routes'
import Table from './table'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

const makeColumns = ({ users, totalCount, tableName, isSmallViewport }) => {
  let columns = [
    {
      Header: 'Name',
      accessor: 'data.name',
    },
    {
      Header: 'Email',
      accessor: 'data.email',
    },
    {
      Header: 'Phone',
      accessor: 'data.phone_number',
    },
    {
      Header: '\u200b',
      accessor: 'id',
      style: { minWidth: 25 },
      Cell: ({ cell: { row: { original: { id } } } }) => (
        <a href={`${routes.ADMIN_CUSTOMERS_PAGE}/${id}`}>
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
      titleIconComponent: <IconUsers color="#fff" />,
      columns: columns,
    },
  ]
}

const TableUsers = ({
  users,
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
  const serverData = React.useMemo(() => users, [users])
  const columns = makeColumns({ users, totalCount, tableName, isSmallViewport })
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

TableUsers.propTypes = {
  users: PropTypes.arrayOf(PropTypes.shape({
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

export default TableUsers
