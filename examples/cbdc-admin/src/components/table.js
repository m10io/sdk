import React, { Fragment, useState, useEffect } from 'react'
import PropTypes from 'prop-types'
import classnames from 'classnames'
import ContentLoader from 'react-content-loader'
import {
  useTable,
  usePagination,
  useExpanded,
} from 'react-table'
import Container from './container'
import Button, {
  BUTTON_THEME_PRIMARY,
} from './button'
import iconDownloadGrey from '../assets/icons/icon-download-grey.svg'
import iconLeftArrowGray from '../assets/icons/icon-arrow-left-gray.svg'
import iconLeftArrowBlue from '../assets/icons/icon-arrow-left-blue.svg'
import iconRightArrowGray from '../assets/icons/icon-arrow-right-gray.svg'
import iconRightArrowBlue from '../assets/icons/icon-arrow-right-black.svg'
import IconAccordion from '../assets/icons/icon-accordion'
import { PRIMARY_COLOR } from '../consts'
import styles from '../styles/table.module.scss'

const PAGE_SIZE_OPTIONS = [10, 20, 30, 40, 50]

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

export const TABLE_HEADER_THEME_DEFAULT = 'TABLE_HEADER_THEME_DEFAULT'
export const TABLE_HEADER_THEME_CARD = 'TABLE_HEADER_THEME_CARD'

const TABLE_THEME_TO_CLASSNAME_MAPPING = {
  [TABLE_HEADER_THEME_DEFAULT]: styles.tableHeaderThemeDefault,
  [TABLE_HEADER_THEME_CARD]: styles.tableHeaderThemeCard,
}

const Table = ({
  columns,
  data = [],
  isLoading,
  onDownload,
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
}) => {
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
          <div className={classnames(
            styles.tableTitleIcon,
            TABLE_THEME_TO_CLASSNAME_MAPPING[headerTheme],
          )}>
            {columns[0].titleIcon && (
              <img src={columns[0].titleIcon || ''} alt={title} />
            )}
            {columns[0].titleIconComponent}
          </div>
          {title}
          <div className={styles.rightHeaderActions}>
            {onDownload && (
              <Button
                theme={BUTTON_THEME_PRIMARY}
                className={styles.downloadButton}
                onClick={onDownload}
              >
                <img src={iconDownloadGrey} alt={'Download'}></img>
                {'Download'}
              </Button>
            )}
            {withAccordion && (
              <div className={styles.accordionExpander} onClick={() => setIsExpanded(!isExpanded)}>
                <IconAccordion color={PRIMARY_COLOR} />
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
                if (!isFirstPage) {
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
              <img
                src={(isFirstPage || isEmpty) ? iconLeftArrowGray : iconLeftArrowBlue}
                alt={'Previous Page'}
              />
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
              <img
                src={(isLastPage || isEmpty) ? iconRightArrowGray : iconRightArrowBlue}
                alt={'Next Page'}
              />
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

const TableWrapper = ({
  loadData,
  isLoading,
  isSmallViewport,
  columnCount = 4,
  ...props
}) => {
  const [pageSize, setPageSize] = useState(50)
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

const TableWithLoadingCheck = ({ isLoading, isSmallViewport, columnCount = 4, ...props }) => {
  if (isLoading) {
    return <TableLoader
      columnCount={isSmallViewport ? 1 : columnCount}
      isSmallViewport={isSmallViewport}
    />
  } else {
    return <TableWrapper isSmallViewport={isSmallViewport} {...props} />
  }
}

Table.propTypes = {
  columns: PropTypes.arrayOf(PropTypes.object.isRequired).isRequired,
  data: PropTypes.arrayOf(PropTypes.object.isRequired).isRequired,
  isLoading: PropTypes.bool,
  onDownload: PropTypes.func,
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
  tablePageIndex: PropTypes.number.isRequired,
  setPageIndex: PropTypes.func.isRequired,
  headerTheme: PropTypes.oneOf([
    TABLE_HEADER_THEME_DEFAULT,
    TABLE_HEADER_THEME_CARD,
  ]).isRequired,
  isSmallViewport: PropTypes.bool,
}

Table.defaultProps = {
  headerTheme: TABLE_HEADER_THEME_DEFAULT,
}

export default TableWithLoadingCheck
