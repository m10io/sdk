import React from 'react'
import PropTypes from 'prop-types'
import classnames from 'classnames'
import { Link } from 'react-router-dom'
import formatMoney from '../utils/format-money'
import styles from '../styles/card.module.scss'
import IconBookOpen from '../assets/icons/icon-book-open'
import Button, {
  BUTTON_THEME_PRIMARY,
} from '../components/button'
import routes from '../routes'
import { PRIMARY_COLOR } from '../consts'

const InfoRow = ({ title, value }) => (
  <div className={styles.infoRow}>
    <div className={styles.topCardTitle}>{title}</div>
    <div className={styles.topCardValue}>{value}</div>
  </div>
)

export const AccountInfoCard = ({ account, accountId }) => {
  return (
    <Card title={`Account - ${accountId}`}>
      <InfoRow title={'Name'} value={account.name} />
      <InfoRow title={'Amount'} value={`$${formatMoney(account.issued)}`} />
      <InfoRow title={'Balance'} value={`$${formatMoney(account.balance)}`} />
      <InfoRow title={'Frozen'} value={(account?.frozen || false).toString()} />
      <Link to={`${routes.ISSUE_PAGE_ROUTE}?issueToAccountId=${accountId}`}>
        <Button
          theme={BUTTON_THEME_PRIMARY}
          text={'Issue To'}
          className={classnames(
            styles.clickableDiv,
            styles.cardButton,
          )}
        />
      </Link>
      <Link to={`${routes.REDEEM_PAGE_ROUTE}?redeemFromAccountId=${accountId}`}>
        <Button
          theme={BUTTON_THEME_PRIMARY}
          text={'Redeem From'}
          className={classnames(
            styles.clickableDiv,
            styles.cardButton,
          )}
        />
      </Link>
    </Card>
  )
}

export const LedgerInfoCard = ({ ledger }) => {
  return (
    <Card title={'Ledger Info'} iconComponent={<IconBookOpen color={PRIMARY_COLOR} />}>
      <InfoRow title={'Current Block Height'} value={ledger.height} />
      <InfoRow title={'URL'} value={ledger.url} />
    </Card>
  )
}

export const CardHeader = ({ title, iconComponent }) => (
  <div className={styles.cardTitle} id={'card-title'}>
    {iconComponent && <div className={styles.cardTitleIcon}>{iconComponent}</div>}
    {title}
  </div>
)

const Card = ({ title, iconComponent, className, children }) => {
  return (
    <div className={classnames(
      styles.cardWrapper,
      className,
    )}>
      {title && <CardHeader title={title} iconComponent={iconComponent} />}
      {children}
    </div>
  )
}

Card.propTypes = {
  title: PropTypes.string,
  icon: PropTypes.object, // imported image (SVG, PNG, etc.)
  className: PropTypes.string,
}

export default Card
