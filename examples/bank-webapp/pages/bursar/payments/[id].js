import React from 'react'
import PropTypes from 'prop-types'
import Page from 'components/page'
// import { useRouter } from 'next/router'
// import { useApi } from 'utils/hooks'
import { PaymentCard } from 'components/info-card'
// import { parsePaymentApi } from 'utils/api'
// import routes from 'routes'

const MOCK_PAYMENT = {
  id: '1',
  senderName: 'Sender Name',
  senderCustomerId: '122',
  receiverName: 'Receiver Name',
  receiverCustomerId: '123',
  timestamp: '2022-05-03T20:23:40.277Z',
  amount: '10000',
  senderInstrument: 'USD',
  txChain: [],
}

const BursarPaymentPage = ({ id, windowWidth }) => {
  // const router = useRouter()
  // TODO: update payment by id url
  // const { isLoading, error, data, isLoaded } = useApi('')
  const payment = MOCK_PAYMENT // parsePaymentApi(data)
  const error = false
  const isLoading = false
  const isLoaded = true
  return (
    <Page
      withGlobalNav
      loadError={error}
      isLoading={isLoading}
      windowWidth={windowWidth}
    >
      {isLoaded && <PaymentCard payment={payment || {}} title={'Bursar Payment'} />}
    </Page>
  )
}

BursarPaymentPage.getInitialProps = async ctx => {
  return { id: ctx.query.id }
}

BursarPaymentPage.propTypes = {
  id: PropTypes.string.isRequired,
  windowWidth: PropTypes.number,
}

export default BursarPaymentPage
