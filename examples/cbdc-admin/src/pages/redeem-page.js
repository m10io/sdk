import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import toast from 'react-hot-toast'
import Page from '../components/page'
import Card from '../components/card'
import Button, {
  BUTTON_THEME_PRIMARY,
} from '../components/button'
import formatMoney from '../utils/format-money'
import IconDollarSign from '../assets/icons/icon-dollar-sign'
import { PRIMARY_COLOR } from '../consts'
import { useQuery } from '../utils/hooks'
import styles from '../styles/shared.module.scss'

const RedeemFormCard = () => {
  const [assets, setAssets] = useState([])
  const [banks, setBanks] = useState([])

  useEffect(() => {
    const getAssets = async() => setAssets(await invoke('get_assets'))
    const getBanks = async() => setBanks(await invoke('get_banks'))
    getAssets()
    getBanks()
  }, [])

  const [bankId, setBankId] = useState('')
  const [assetAccountId, setAssetAccountId] = useState('')
  const [amount, setAmount] = useState('')

  const query = useQuery()
  const redeemFromAccountId = query.get('redeemFromAccountId')
  useEffect(() => {
    if (redeemFromAccountId) setAssetAccountId(redeemFromAccountId)
  }, [redeemFromAccountId])

  const selectedAsset = assets.find(asset => asset.accountId === assetAccountId)
  const selectedBank = banks.find(bank => bank.accountId === bankId)

  const redeemMoney = async e => {
    e.preventDefault()
    try {
      await invoke('redeem_to_account', {
        parentAccount: assetAccountId,
        account: bankId,
        amount: Number(amount),
        reference: 'Redeem Money'
      })
      setAmount('')
      toast.success('Redeemed Money')
    } catch (e) {
      toast.error(e.toString())
    }
  }

  return (
    <Card title={'Redeem'} iconComponent={<IconDollarSign color={PRIMARY_COLOR} />}>
      <form onSubmit={redeemMoney} className={styles.formWrapper}>
        <div className={styles.flexDiv}>
          <label>
            <div>{'From Bank'}</div>
            <select
              value={bankId}
              onChange={e => setBankId(e.target.value)}
              name={'bank-id'}
              className={styles.textInput}
            >
              <option />
              {banks.map((bank, i) => (
                <option value={bank.accountId} key={i}>{bank.name}</option>
              ))}
            </select>
          </label>
          {selectedBank && (
            <label>
              {'Issued'}
              <div className={styles.addedInfoText}>
                {formatMoney(selectedBank?.issued)}
              </div>
            </label>
          )}
        </div>
        <div className={styles.flexDiv}>
          <label>
            <div>{'To Asset'}</div>
            <select
              value={assetAccountId}
              onChange={e => setAssetAccountId(e.target.value)}
              name={'asset-code'}
              className={styles.textInput}
            >
              <option />
              {assets.map((asset, i) => (
                <option value={asset.accountId} key={i}>{asset.name}</option>
              ))}
            </select>
          </label>
          {selectedAsset && (
            <label>
              {'Issued'}
              <div className={styles.addedInfoText}>
                {formatMoney(selectedAsset?.issued)}
              </div>
            </label>
          )}
        </div>
        <div>
          <label>
            <div>{'Amount'}</div>
            <input
              value={amount}
              onChange={e => setAmount(e.target.value)}
              type={'Number'}
              name={'amount'}
              className={styles.textInput}
            />
          </label>
        </div>
        <div>
          <Button theme={BUTTON_THEME_PRIMARY} type={'submit'} text={'Submit'} />
        </div>
      </form>
    </Card>
  )
}

const RedeemPage = ({ windowWidth }) => {
  return (
    <Page
      withGlobalNav
      windowWidth={windowWidth}
      includeBackButton
    >
      <RedeemFormCard />
    </Page>
  )
}

export default RedeemPage
