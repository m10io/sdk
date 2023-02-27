import React from 'react'
import PropTypes from 'prop-types'
import classnames from 'classnames'
import styles from './styles/status-icon.module.scss'

export const STATUS_ICON_TYPE_WITHDRAWAL = 'STATUS_ICON_TYPE_WITHDRAWAL'
export const STATUS_ICON_TYPE_DEPOSIT = 'STATUS_ICON_TYPE_DEPOSIT'
export const STATUS_ICON_TYPE_SUCCESS = 'STATUS_ICON_TYPE_SUCCESS'
export const STATUS_ICON_TYPE_VERIFIED = 'STATUS_ICON_TYPE_VERIFIED'
export const STATUS_ICON_TYPE_PENDING = 'STATUS_ICON_TYPE_PENDING'
export const STATUS_ICON_TYPE_ERROR = 'STATUS_ICON_TYPE_ERROR'
export const STATUS_ICON_TYPE_FAILED = 'STATUS_ICON_TYPE_FAILED'
export const STATUS_ICON_TYPE_UNKNOWN = 'STATUS_ICON_TYPE_UNKNOWN'
export const STATUS_ICON_TYPE_OPEN = 'STATUS_ICON_TYPE_OPEN'
export const STATUS_ICON_TYPE_FRAUD = 'STATUS_ICON_TYPE_FRAUD'

const STATUS_ICON_TYPE_TO_CLASSNAME_MAPPING = {
  [STATUS_ICON_TYPE_WITHDRAWAL]: styles.statusIconTypeWithdrawal,
  [STATUS_ICON_TYPE_DEPOSIT]: styles.statusIconTypeDeposit,
  [STATUS_ICON_TYPE_SUCCESS]: styles.statusIconTypeSuccess,
  [STATUS_ICON_TYPE_OPEN]: styles.statusIconTypeOpen,
  [STATUS_ICON_TYPE_VERIFIED]: styles.statusIconTypeVerified,
  [STATUS_ICON_TYPE_PENDING]: styles.statusIconTypePending,
  [STATUS_ICON_TYPE_ERROR]: styles.statusIconTypeError,
  [STATUS_ICON_TYPE_FAILED]: styles.statusIconTypeFailed,
  [STATUS_ICON_TYPE_UNKNOWN]: styles.statusIconTypeUnknown,
  [STATUS_ICON_TYPE_FRAUD]: styles.statusIconTypeFraud,
}

const STATUS_ICON_TYPE_TO_TEXT_MAPPING = {
  [STATUS_ICON_TYPE_WITHDRAWAL]: 'Withdrawal',
  [STATUS_ICON_TYPE_DEPOSIT]: 'Deposit',
  [STATUS_ICON_TYPE_SUCCESS]: 'Success',
  [STATUS_ICON_TYPE_OPEN]: 'Open',
  [STATUS_ICON_TYPE_VERIFIED]: 'Verified',
  [STATUS_ICON_TYPE_PENDING]: 'Pending',
  [STATUS_ICON_TYPE_ERROR]: 'Error',
  [STATUS_ICON_TYPE_FAILED]: 'Failed',
  [STATUS_ICON_TYPE_UNKNOWN]: 'Unknown',
  [STATUS_ICON_TYPE_FRAUD]: 'Fraud',
}

export const STATUS_ICON_TEXT_TO_ICON_TYPE_MAPPING =
  Object
    .fromEntries(Object.entries(STATUS_ICON_TYPE_TO_TEXT_MAPPING)
    .map(a => a.reverse()))

const StatusIcon = ({ type, className, onClick }) => (
  <div
    onClick={onClick}
    className={classnames(
      styles.statusIconWrapper,
      STATUS_ICON_TYPE_TO_CLASSNAME_MAPPING[type],
      className,
    )}
  >
    {STATUS_ICON_TYPE_TO_TEXT_MAPPING[type]}
  </div>
)

StatusIcon.propTypes = {
  type: PropTypes.oneOf(Object.keys(STATUS_ICON_TYPE_TO_CLASSNAME_MAPPING)).isRequired,
  className: PropTypes.string,
  onClick: PropTypes.func,
}

export default StatusIcon
