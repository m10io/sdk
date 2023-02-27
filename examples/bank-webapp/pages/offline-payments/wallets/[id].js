import React from 'react'
import moment from 'moment'
import PropTypes from 'prop-types'
import Page from 'components/page'
import Card from 'components/card'
import { InfoRow } from 'components/info-card'
import { useApi } from 'utils/hooks'
import IconBookOpen from 'assets/icons/icon-book-open'
import TableOfflinePayments from 'components/table-offline-payments'
import routes from 'routes'
import ChainView from '../../../components/chain-view'

const WalletCard = ({ data = {} }) => {
  const createdAt = moment(data?.created_at, 'YYYY-MM-DD hh:mm:ss').format('MMM Do YYYY, h:mm:ss a')
  return (
    <Card title={'Wallet'} iconComponent={<IconBookOpen color={'#fff'} />}>
      <InfoRow title={'Display Name'} value={data.display_name} />
      <InfoRow title={'Currency'} value={data.currency} />
      <InfoRow title={'Date Created'} value={createdAt} />
      <InfoRow title={'ID'} value={data.id} />
    </Card>
  )
}

const OfflinePaymentWalletPage = ({ id, windowWidth }) => {
  const { isLoading, error, data, isLoaded } = useApi(`${routes.OFFLINE_PAYMENTS_WALLETS_API}/${id}`)
  const { isLoading: isLoadingPayments, data: paymentsData } = useApi(`${routes.OFFLINE_PAYMENTS_WALLETS_API}/${id}/payments`)
  const { isLoading: isLoadingFraud, data: fraudData } = useApi(`${routes.OFFLINE_PAYMENTS_FRAUD_API}?wallet=${id}`)
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
        <>
          <WalletCard data={data} />
          <TableOfflinePayments
            payments={paymentsData?.data || []}
            fraudPayments={fraudData?.data || []}
            isLoading={isLoadingPayments}
            tableName={'Offline Payments'}
            windowWidth={windowWidth}
            noPagination
            includeFraudPaymentsToggle
          />
          <ChainView
            windowWidth={windowWidth * 0.7 }
            isLoading={isLoadingPayments || isLoadingFraud}
            payments={paymentsData?.data || []}
            fraud={fraudData?.data || []}
          />
        </>
      )}
    </Page>
  )
}

OfflinePaymentWalletPage.getInitialProps = async ctx => {
  return { id: ctx.query.id }
}

OfflinePaymentWalletPage.propTypes = {
  id: PropTypes.string.isRequired,
  windowWidth: PropTypes.number,
}

export default OfflinePaymentWalletPage
