import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import toast from 'react-hot-toast'
import Page from '../components/page'
import { AccountInfoCard } from '../components/card'
import { useParams } from 'react-router-dom'

const AccountsPage = ({ windowWidth }) => {
  const { accountId } = useParams()
  const [account, setAccount] = useState([])

  useEffect(() => {
    async function getAccount() {
      try {
        setAccount(await invoke('get_account', { id: accountId }))
      } catch (e) {
        toast.error(e.toString())
      }
    }
    getAccount()
  }, [accountId])

  return (
    <Page
      withGlobalNav
      windowWidth={windowWidth}
      includeBackButton
    >
      <AccountInfoCard account={account} accountId={accountId} />
    </Page>
  )
}

export default AccountsPage
