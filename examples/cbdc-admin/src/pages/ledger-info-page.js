import { useState, useEffect } from 'react'
import Page from '../components/page'
import { LedgerInfoCard } from '../components/card'
import { invoke } from '@tauri-apps/api/tauri'
import styles from '../styles/shared.module.scss'

function LedgerInfoPage() {
  const [blockHeight, updateBlockHeight] = useState('')
  const [ledgerInfo, updateLedgerInfo] = useState({})

  useEffect(() => {
    async function getBlockHeight() {
      updateBlockHeight(await invoke('block_height'))
    }
    getBlockHeight()

    async function getLedgerInfo() {
      updateLedgerInfo(await invoke('ledger_info'))
    }
    getLedgerInfo()
  }, [])

  return (
    <div>
      <header className={styles.app}>
        <Page
          withGlobalNav
          includeBackButton
        >
          <LedgerInfoCard ledger={{
            height: blockHeight,
            url: ledgerInfo.url
          }} />
        </Page>
      </header>
    </div>
  )
}

export default LedgerInfoPage
