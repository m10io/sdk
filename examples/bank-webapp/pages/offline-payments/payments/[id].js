import React from 'react'
import moment from 'moment'
import PropTypes from 'prop-types'
import Link from 'components/link'
import Page from 'components/page'
import Card from 'components/card'
import { InfoRow } from 'components/info-card'
import { useApi } from 'utils/hooks'
import IconDollarSign from 'assets/icons/icon-book-open'
import routes from 'routes'
import formatMoney from 'utils/format-money'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()
const CURRENCY = publicRuntimeConfig.bankAsset

const parseDate = date => date == null ? 'Pending' : moment(date, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')

const OfflinePaymentCard = ({ data = {} }) => {
  return (
    <Card title={'Payment'} iconComponent={<IconDollarSign color={'#000000'} />}>
      <InfoRow title={'Sender'} value={(
        <Link href={`${routes.OFFLINE_PAYMENTS_WALLETS_PAGE}/${data.from_wallet}`}>
          {data.from_display_name}
        </Link>
      )} />
      <InfoRow title={'Receiver'} value={(
        <Link href={`${routes.OFFLINE_PAYMENTS_WALLETS_PAGE}/${data.to_wallet}`}>
          {data.to_display_name}
        </Link>
      )} />
      <InfoRow title={'Amount'} value={`${formatMoney(data.amount)} ${data.currency}`} />
      <InfoRow title={'Date uploaded'} value={parseDate(data?.uploaded_at)} />
      <InfoRow title={'Debit date'} value={parseDate(data?.debited_at)} />
      <InfoRow title={'Credit date'} value={parseDate(data?.credited_at)} />
      <InfoRow title={'ID'} value={data.id} />
    </Card>
  )
}

const OfflinePaymentPage = ({ id, windowWidth }) => {
  const { isLoading, data, isLoaded, error } = useApi(`${routes.OFFLINE_PAYMENTS_PAYMENTS_API}?currency=${CURRENCY}`)
  const payment = (data?.data || []).find(p => p.id === Number(id))
  return (
    <Page
      withGlobalNav
      withSidebar
      loadError={error}
      isLoading={isLoading}
      windowWidth={windowWidth}
      navLogoRoute={routes.OFFLINE_PAYMENTS_PAYMENTS_PAGE}
    >
      {isLoaded && (
        <OfflinePaymentCard data={payment} />
      )}
    </Page>
  )
}

OfflinePaymentPage.getInitialProps = async ctx => {
  return { id: ctx.query.id }
}

OfflinePaymentPage.propTypes = {
  id: PropTypes.string.isRequired,
  windowWidth: PropTypes.number,
}

export default OfflinePaymentPage
