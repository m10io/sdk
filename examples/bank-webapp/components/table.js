import React, { Fragment, useState, useEffect } from 'react'
import PropTypes from 'prop-types'
import classnames from 'classnames'
import ContentLoader from 'react-content-loader'
import {
  useTable,
  usePagination,
  useExpanded,
} from 'react-table'
import Container from 'components/container'
import Image from 'next/image'
import IconAccordion from 'assets/icons/icon-accordion'
import IconArrowRight from 'assets/icons/icon-arrow-right'
import IconArrowLeft from 'assets/icons/icon-arrow-left'
import styles from './styles/table.module.scss'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

const PAGE_SIZE_OPTIONS = [10, 25, 50, 100]

const loaderProps = {
  backgroundColor: '#f3f3f3',
  foregroundColor: '#ecebeb',
  speed: 2,
}

const TableLoader = ({ columnCount, isSmallViewport }) => (
  <Container>
    <div className={styles.table}>
      <div className={styles.tableTitleRow}>
        <ContentLoader
          width={isSmallViewport ? 200 : 310}
          height={25}
          viewBox={`0 0 ${isSmallViewport ? 200 : 310} 25`}
          {...loaderProps}
        >
          <rect x={'40'} y={'2'} rx={'4'} ry={'4'} width={'240'} height={'20'} />
          <rect x={'0'} y={'0'} rx={'4'} ry={'4'} width={'25'} height={'25'} />
        </ContentLoader>
      </div>
      <table>
        <tbody>
          {Array.from(Array(columnCount)).map((iCell, i) => (
            <tr key={i}>
              {Array.from(Array(columnCount)).map((jCell, j) => (
                <td key={j}>
                  <ContentLoader
                    width={isSmallViewport ? 200 : 100}
                    height={20}
                    viewBox={`0 0 ${isSmallViewport ? 200 : 100} 20`}
                    {...loaderProps}
                  >
                    <rect x={'0'} y={'0'} rx={'4'} ry={'4'} width={isSmallViewport ? '200' : '100'} height={'20'} />
                  </ContentLoader>
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
      <div className={classnames(
        styles.tableBottomRow,
        styles.tableBottomRowNoPagination,
      )} />
    </div>
  </Container>
)

const TABLE_HEADER_THEME_DEFAULT = 'TABLE_HEADER_THEME_DEFAULT'
export const TABLE_HEADER_THEME_CARD = 'TABLE_HEADER_THEME_CARD'
export const TABLE_HEADER_THEME_NONE = 'TABLE_HEADER_THEME_NONE'

const TABLE_THEME_TO_CLASSNAME_MAPPING = {
  [TABLE_HEADER_THEME_DEFAULT]: styles.tableHeaderThemeDefault,
  [TABLE_HEADER_THEME_CARD]: styles.tableHeaderThemeCard,
  [TABLE_HEADER_THEME_NONE]: styles.tableHeaderThemeNone,
}

const Table = ({
  columns,
  data = [],
  isLoading,
  renderRowSubComponent,
  rowSubComponentProps,
  noPagination,
  noTitleRow,
  noTableHeaders,
  tableClassName,
  formFieldOptions,
  onFormOptionSelected,
  titleRowClassName,
  topRowClassName,
  columnCount = 4,
  expandedRows = {},
  loadData,
  limit,
  updatePageSize,
  nextPageToken,
  withAccordion,
  headerTheme,
  tablePageIndex,
  setPageIndex,
  isSmallViewport,
  onlyForwardPagination,
}) => {
  if (isLoading) {
    return <TableLoader
      columnCount={isSmallViewport ? 1 : columnCount}
      isSmallViewport={isSmallViewport}
    />
  }

  const isEmpty = data.length === 0
  // TODO: add UI for no data

  const {
    getTableProps,
    getTableBodyProps,
    headerGroups,
    prepareRow,
    page, // 'page' prop includes only active 'rows'
    // visibleColumns,
    setPageSize,
  } = useTable(
    {
      columns,
      data,
      initialState: {
        // pageIndex: 0,
        expanded: expandedRows,
      },
    },
    useExpanded,
    usePagination,
  )

  const [isExpanded, setIsExpanded] = useState(true)

  useEffect(() => {
    setPageSize(limit)
  }, [limit])

  const title = headerGroups[0].headers[0].Header
  const isFirstPage = tablePageIndex === 0
  const isLastPage = data.length < limit

  return (
    <Container className={styles.table}>
      {!noTitleRow && (
        <div
          className={classnames(
            styles.tableTitleRow,
            titleRowClassName,
            TABLE_THEME_TO_CLASSNAME_MAPPING[headerTheme],
          )}
          id={'table-title-row'}
        >
          {headerTheme !== TABLE_HEADER_THEME_NONE && (
            <div className={classnames(
              styles.tableTitleIcon,
              TABLE_THEME_TO_CLASSNAME_MAPPING[headerTheme],
            )}>
              {columns[0].titleIcon && (
                <Image src={columns[0].titleIcon || ''} alt={title} />
              )}
              {columns[0].titleIconComponent}
            </div>
          )}
          {headerTheme !== TABLE_HEADER_THEME_NONE && (
            title
          )}
          <div className={styles.rightHeaderActions}>
            {withAccordion && (
              <div className={styles.accordionExpander} onClick={() => setIsExpanded(!isExpanded)}>
                <IconAccordion color={publicRuntimeConfig.bankPrimaryColor} />
              </div>
            )}
          </div>
        </div>
      )}
      {isExpanded && (
        <table {...getTableProps()} className={classnames(tableClassName)}>
          {!noTableHeaders && (
            <thead>
              {headerGroups.map((headerGroup, i) => {
                const isTitleRow = i === 0
                return (!isTitleRow && (
                  <tr {...headerGroup.getHeaderGroupProps()} key={i}>
                    {headerGroup.headers.map((column, j) => {
                      return (
                        <th {...column.getHeaderProps()} style={column.style} key={j}>
                          {column.render('Header')}
                        </th>
                      )
                    })}
                  </tr>
                ))
              })}
            </thead>
          )}
          <tbody {...getTableBodyProps()}>
            {page.map((row, index) => {
              prepareRow(row)
              return (
                <Fragment key={index}>
                  <tr {...row.getRowProps()}>
                    {row.cells.map(cell => {
                      const { id } = cell.row.original
                      const cellClassName = classnames(
                        styles.cellStandard,
                        index === 0 && topRowClassName,
                      )
                      return (
                        <td
                          className={cellClassName}
                          key={id}
                          style={cell.column.style}
                          {...cell.getCellProps()}
                        >
                          {cell.render('Cell')}
                        </td>
                      )
                    })}
                  </tr>
                  {renderRowSubComponent && row.isExpanded && (
                    renderRowSubComponent({ row, ...rowSubComponentProps, isSmallViewport })
                  )}
                </Fragment>
              )
            })}
          </tbody>
        </table>
      )}
      {(!isExpanded || noPagination)
        ? (
          <div className={classnames(
            styles.tableBottomRow,
            styles.tableBottomRowNoPagination,
          )} />
        ) : (
          <div className={styles.tableBottomRow}>
            <div
              onClick={() => {
                if (!isFirstPage && !onlyForwardPagination) {
                  const backwardsId = data[0].id + limit
                  loadData({ limit, id: backwardsId, time: nextPageToken?.time })
                  setPageIndex(tablePageIndex - 1)
                }
              }}
              role={'button'}
              tabIndex={0}
              disabled={isFirstPage}
              className={classnames(
                styles.tablePageNumberLink,
                isFirstPage && styles.tablePageNumberLinkDisabled,
              )}
            >
              <IconArrowLeft color={(isFirstPage || isEmpty) ? '#CCCCCC' : publicRuntimeConfig.bankPrimaryColor} />
            </div>
            <div className={styles.tableBottomRowArrowsDivider} />
            <div
              onClick={() => {
                if (!isLastPage) {
                  loadData({ limit, id: nextPageToken?.id || nextPageToken, time: nextPageToken?.time })
                  setPageIndex(tablePageIndex + 1)
                }
              }}
              disabled={isLastPage}
              role={'button'}
              tabIndex={0}
              className={styles.tablePageNumberLink}
              className={classnames(
                styles.tablePageNumberLink,
                isLastPage && styles.tablePageNumberLinkDisabled,
              )}
            >
              <IconArrowRight color={(isLastPage || isEmpty) ? '#CCCCCC' : publicRuntimeConfig.bankPrimaryColor} />
            </div>
            <div className={styles.tableSizeSelectorWrapper}>
              <select value={limit} onChange={updatePageSize}>
                {PAGE_SIZE_OPTIONS.map(sizeOption => ((
                  <option value={sizeOption} key={sizeOption}>
                    {`${sizeOption} per page`}
                  </option>
                )))}
              </select>
            </div>
          </div>
        )
      }
    </Container>
  )
}

const TableWrapper = ({ loadData, limit, ...props }) => {
  const [pageSize, setPageSize] = useState(limit || PAGE_SIZE_OPTIONS[0])
  const updatePageSize = async e => {
    const limit = e.target.value
    await loadData({ limit: Number(limit) })
    setPageIndex(0)
    setPageSize(limit)
  }
  const [tablePageIndex, setPageIndex] = useState(0)

  return (
    <Table
      updatePageSize={updatePageSize}
      limit={pageSize}
      loadData={loadData}
      tablePageIndex={tablePageIndex}
      setPageIndex={setPageIndex}
      {...props}
    />
  )
}

Table.propTypes = {
  columns: PropTypes.arrayOf(PropTypes.object.isRequired).isRequired,
  data: PropTypes.arrayOf(PropTypes.object.isRequired).isRequired,
  isLoading: PropTypes.bool,
  renderRowSubComponent: PropTypes.func,
  noPagination: PropTypes.bool,
  noTitleRow: PropTypes.bool,
  noTableHeaders: PropTypes.bool,
  tableClassName: PropTypes.string,
  formFieldOptions: PropTypes.arrayOf(PropTypes.object),
  onFormOptionSelected: PropTypes.func,
  rowSubComponentProps: PropTypes.object,
  titleRowClassName: PropTypes.string,
  topRowClassName: PropTypes.string,
  expandedRows: PropTypes.object,
  nextPageToken: PropTypes.oneOfType([
    PropTypes.shape({
      id: PropTypes.number,
      time: PropTypes.string,
    }),
    PropTypes.string,
    PropTypes.number,
  ]),
  tablePageIndex: PropTypes.number.isRequired,
  setPageIndex: PropTypes.func.isRequired,
  headerTheme: PropTypes.oneOf([
    TABLE_HEADER_THEME_DEFAULT,
    TABLE_HEADER_THEME_CARD,
    TABLE_HEADER_THEME_NONE,
  ]).isRequired,
  isSmallViewport: PropTypes.bool,
  onlyForwardPagination: PropTypes.bool,
  limit: PropTypes.number,
}

Table.defaultProps = {
  headerTheme: TABLE_HEADER_THEME_DEFAULT,
}

export default TableWrapper
