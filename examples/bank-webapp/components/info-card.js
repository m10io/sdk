import React from 'react'
import moment from 'moment'
import Card from 'components/card'
import IconUser from 'icons/icon-user'
import IconDollarSign from 'icons/icon-dollar-sign'
import styles from './styles/info-card.module.scss'
import formatMoney from 'utils/format-money'
import getConfig from 'next/config'
import StatusIcon, {
  STATUS_ICON_TYPE_OPEN,
} from 'components/status-icon'
const { publicRuntimeConfig } = getConfig()
// NOTE: all moment deprecation warnings are due to receiving initial null values
moment.suppressDeprecationWarnings = true

export const InfoRow = ({ title, value }) => (
  <div className={styles.infoRow}>
    <div className={styles.topCardTitle}>{title}</div>
    <div className={styles.topCardValue}>{value}</div>
  </div>
)

export const CustomerInfoCard = ({ customer = {} }) => {
  const createdAt = moment(customer?.created_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
  return (customer?.id ? (
    <Card title={'Customer Info'} iconComponent={<IconUser color={publicRuntimeConfig.bankPrimaryColor} />}>
      <div className={styles.cardColumn}>
        {customer.contact_data?.name && <InfoRow title={'Name'} value={customer.contact_data?.name} />}
        {customer.contact_data?.email && <InfoRow title={'Email'} value={customer.contact_data?.email} />}
        {customer.account_id && <InfoRow title={'Account ID'} value={customer.account_id} />}
        {customer['description-of-services'] && (
          <InfoRow title={'Description of Services'} value={customer['description-of-services']} />
        )}
        <InfoRow title={'Date Created'} value={createdAt} />
        {customer.contact_type && <InfoRow title={'Contact Type'} value={customer.contact_type} />}
      </div>
    </Card>
  ) : null)
}

export const UserCard = ({ title, user, isLoaded }) => {
  const phone = user?.attributes?.phone || user.phone_number
  return (
    <Card title={title || 'User Info'} iconComponent={<IconUser color={publicRuntimeConfig.bankPrimaryColor} />}>
      <InfoRow title={'Name'} value={user?.name} />
      <InfoRow title={'Email'} value={user?.email} />
      {phone && <InfoRow title={'Phone'} value={phone} />}
      {user?.picture && (
        <InfoRow title={'Picture'} value={<img src={user?.picture} />} />
      )}
    </Card>
  )
}

export const AccountCard = ({ account = {} }) => {
  const bank = account?.bank_reference || {}
  const title = account?.id ? `Account - ${account?.id}` : 'Account'
  const updatedAt = moment(account?.updated_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
  const createdAt = moment(account?.created_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
  // TODO: show icons for other account status types
  const statusValue = account?.status === 'Open' ? <StatusIcon type={STATUS_ICON_TYPE_OPEN} /> : account?.status
  return (
    <Card title={title} iconComponent={<IconDollarSign color={publicRuntimeConfig.bankPrimaryColor} />}>
      <InfoRow title={'Bank Name'} value={bank?.name} />
      <InfoRow title={'Currency'} value={bank?.currency} />
      <InfoRow title={'Status'} value={statusValue} />
      <InfoRow title={'Tenant'} value={account?.tenant} />
      <InfoRow title={'Last Updated'} value={updatedAt} />
      <InfoRow title={'Date Created'} value={createdAt} />
    </Card>
  )
}

export const PaymentCard = ({ payment, title }) => {
  const txChain = Array.isArray(payment?.txChain) ? payment?.txChain : []
  const senderBank = txChain[0]?.sender?.bank
  const receiverBank = txChain[0]?.receiver?.bank
  // const senderAccountId = txChain[0]?.sender?.account_id
  // const receiverAccountId = txChain[0]?.receiver?.account_id
  const memo = txChain[0]?.memo
  return (
    <Card title={title || 'Payment'} iconComponent={<IconDollarSign color={publicRuntimeConfig.bankPrimaryColor} />}>
      <InfoRow title={'Amount'} value={`${formatMoney(payment?.amount)} ${payment?.senderInstrument}`} />
      <InfoRow title={'Date/Time'} value={moment(payment?.timestamp, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')} />
      {payment?.memo && (
        <InfoRow title={'Memo'} value={payment?.memo} />
      )}
      <InfoRow title={'Sender Name'} value={payment?.senderName} />
      {senderBank && (
        <InfoRow title={'Sender Bank'} value={senderBank} />
      )}
      <InfoRow title={'Receiver Name'} value={payment?.receiverName} />
      {receiverBank && (
        <InfoRow title={'Receiver Bank'} value={receiverBank} />
      )}
      {memo && (
        <InfoRow title={'Memo'} value={memo} />
      )}
    </Card>
  )
}
