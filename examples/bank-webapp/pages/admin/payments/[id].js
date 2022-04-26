import React from 'react'
import PropTypes from 'prop-types'
import Page from 'components/page'
import { useRouter } from 'next/router'
import { useApi } from 'utils/hooks'
import { PaymentCard } from 'components/info-card'
import { parsePaymentApi } from 'utils/api'
import routes from 'routes'

const PaymentPage = ({ id, windowWidth }) => {
  const router = useRouter()
  const { asset, accountId } = router.query
  const { isLoading, error, data, isLoaded } = useApi(`${routes.PAYMENTS_API}/${id}?asset=${asset}&accountId=${accountId}`)
  const payment = parsePaymentApi(data)
  return (
    <Page
      withSidebar
      withGlobalNav
      loadError={error}
      isLoading={isLoading}
      windowWidth={windowWidth}
    >
      {isLoaded && (
        <>
          <PaymentCard payment={payment || {}} />
        </>
      )}
    </Page>
  )
}

PaymentPage.getInitialProps = async ctx => {
  return { id: ctx.query.id }
}

PaymentPage.propTypes = {
  id: PropTypes.string.isRequired,
  windowWidth: PropTypes.number,
}

export default PaymentPage
