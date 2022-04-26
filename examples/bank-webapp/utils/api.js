// parse responses from /payments api for PaymentsTable component
export const parsePaymentsApi = (data = []) => {
  const transfersFiltered = data.filter(d => !!d.transfers.length)
  const transfers = transfersFiltered.map(t => {
    const senderTx = t.transfers[0] || {}
    const receiverTx = t.transfers[t.transfers.length - 1] || {}
    return ({
      id: t.id,
      senderName: senderTx?.sender?.name ?? '',
      senderCustomerId: senderTx?.sender?.customerId ?? '',
      receiverName: receiverTx?.receiver?.name ?? '',
      receiverCustomerId: receiverTx?.receiver?.customerId ?? '',
      timestamp: senderTx.timestamp ?? '',
      amount: senderTx.amount ?? '',
      senderInstrument: (senderTx.instrument || '').toUpperCase() ?? '',
      txChain: t.transfers || [],
    })
  }).filter(p => !!p.txChain.length)
  return transfers
}

export const parsePaymentApi = (t = {}) => {
  const transfers = Array.isArray(t?.transfers) ? t?.transfers : []
  const senderTx = transfers[0] || {}
  const receiverTx = transfers[transfers.length - 1] || {}
  return ({
    id: t?.id,
    senderName: senderTx?.sender?.name ?? '',
    senderCustomerId: senderTx?.sender?.customerId ?? '',
    receiverName: receiverTx?.receiver?.name ?? '',
    receiverCustomerId: receiverTx?.receiver?.customerId ?? '',
    timestamp: senderTx.timestamp ?? '',
    amount: senderTx.amount ?? '',
    senderInstrument: (senderTx.instrument || '').toUpperCase() ?? '',
    txChain: transfers || [],
  })
}
