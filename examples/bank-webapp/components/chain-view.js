import React from 'react'
import classnames from 'classnames'
import ReactFlow, { Controls, MarkerType } from 'react-flow-renderer'
import { LoadingSpinner } from './button'
import Container from './container'
import formatMoney from 'utils/format-money'
import styles from './styles/chain-view.module.scss'

const CHAIN_VIEW_HEIGHT = '30vh'

const nodeId = p => `${p.from_wallet || p.wallet_id}-${p.from_seq_id || p.seq_id}`
const fraudId = f => `${nodeId(f)}-fraud`
const chainPosition = ({ i }) => ({ x: 250 * i, y: 0 })
const fraudPosition = (fraud, payments, previousFraudNodeCount) => {
  const matchingEntry = payments.findIndex(p => p.from_wallet === p.wallet_id && p.from_seq_id === fraud.seq_id)
  if (matchingEntry >= 0) {
    return ({ x: chainPosition(matchingEntry).x, y: 200 })
  } else {
    return { x: 100 * previousFraudNodeCount, y: 200 }
  }
}

const paymentNode = (payment, index) => ({
  id: nodeId(payment),
  sourcePosition: 'right',
  targetPosition: 'left',
  position: chainPosition({ i: index }),
  data: {
    label: (
        <div className={styles.nodeLabel}>
          <div className={styles.nodeLabelRecipients}>
            {`${payment.from_display_name} -> ${payment.to_display_name}`}
          </div>
          <div className={styles.nodeLabelAmount}>
            {`$${formatMoney(payment.amount)} ${payment.currency}`}
          </div>
        </div>
      )
  }
})

const paymentEdge = (payment, next) => ({
  id: `${nodeId(payment)}-edge`,
  source: nodeId(payment),
  type: 'smoothstep',
  target: nodeId(next),
})

const fraudNode = (fraud, payments, previousFraudNodeCount) => ({
  id: fraudId(fraud),
  sourcePosition: 'bottom',
  // NOTE @sadroeck: if not left/bottom, the label seems to be hidden for some reason..
  targetPosition: 'left',
  position: fraudPosition(fraud, payments, previousFraudNodeCount),
  style: { backgroundColor: 'rgba(255, 0, 0, 0.3)' },
  data: {
 label: (
    <div className={styles.nodeLabel}>
      <div className={styles.nodeLabelRecipients}>
        {`${fraud.from_display_name || fraud.from_wallet} -> ${fraud.to_display_name || fraud.to_wallet}`}
      </div>
      <div className={classnames(
        styles.nodeLabelAmount,
        styles.nodeLabelAmountFraud,
      )}>
        {`$${formatMoney(fraud.amount)} ${fraud.currency}`}
      </div>
    </div>
  )
},
})

const fraudEdge = fraud => ({
  id: `${nodeId(fraud)}-fraud-edge`,
  source: nodeId(fraud),
  type: 'smoothstep',
  label: 'FRAUD',
  animated: true,
  labelStyle: { fill: 'red', fontWeight: 600 },
  style: { stroke: 'red' },
  target: fraudId(fraud),
})

const ChainView = ({ hideControls, windowWidth, payments, fraud, isLoading }) => {
  if (isLoading) {
    return (
      <div className={styles.dynamicButtonLoadingSpinner}>
        {LoadingSpinner}
      </div>
    )
  }

  const paymentNodes = payments.map(paymentNode)
  const fraudNodes = fraud.map((f, i) => fraudNode(f, payments, i))
  const paymentEdges = pairwise(payments, paymentEdge)
  const fraudEdges = fraud.map(fraudEdge)

  return (
    <Container>
        <div style={{ height: CHAIN_VIEW_HEIGHT, width: windowWidth }}>
            <ReactFlow
            defaultNodes={paymentNodes.concat(fraudNodes)}
            defaultEdges={paymentEdges.concat(fraudEdges)}
            defaultEdgeOptions={{
              animated: true,
              markerEnd: {
                type: MarkerType.Arrow,
              },
            }}
            nodesDraggable={false}
            draggable={false}
            fitView
            attributionPosition="bottom-left"
            >
                { (hideControls) ? <></> : <Controls /> }
            </ReactFlow>
        </div>
    </Container>
  )
}

const pairwise = (arr, func) => {
  const output = []
  for (let i = 0; i < arr.length - 1; i++) {
      output.push(func(arr[i], arr[i + 1]))
  }
  return output
}

export default ChainView
