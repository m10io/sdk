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

const makeColumns = ({ customers, totalCount, tableName }) => ([
  {
    Header: tableName,
    titleIconComponent: <IconUsers color="#fff" />,
    columns: [
      {
        Header: 'Name',
        accessor: 'contact_data.name',
      },
      {
        Header: 'Email',
        accessor: 'contact_data.email',
      },
      {
        Header: '\u200b',
        accessor: 'id',
        style: { minWidth: 25, textAlign: 'right' },
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
    ],
  },
])

const TableContacts = ({
  customers,
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
  const serverData = React.useMemo(() => customers, [customers])
  const columns = makeColumns({ customers, totalCount, tableName, isSmallViewport })
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

TableContacts.propTypes = {
  customers: PropTypes.arrayOf(PropTypes.shape({
    contact_data: PropTypes.shape({
      name: PropTypes.string,
      email: PropTypes.string,
    }),
    id: PropTypes.number,
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

export default TableContacts
