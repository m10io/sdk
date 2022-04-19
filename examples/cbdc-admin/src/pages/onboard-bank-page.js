import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import toast from 'react-hot-toast'
import Page from '../components/page'
import TableAccounts from '../components/table-accounts'
import IconBookOpen from '../assets/icons/icon-book-open'
import { PRIMARY_COLOR } from '../consts'
import Card from '../components/card'
import formatMoney from '../utils/format-money'
import Button, {
  BUTTON_THEME_PRIMARY,
} from '../components/button'
import { useLocation, useNavigate } from 'react-router-dom'
import routes from '../routes'
import styles from '../styles/shared.module.scss'

const CreateBankCard = ({ redirectUponSubmission, getBanks }) => {
  const { pathname } = useLocation()
  const navigate = useNavigate()

  const [assets, updateAssets] = useState([])

  useEffect(() => {
    async function getAssets() {
      updateAssets(await invoke('get_assets'))
    }
    getAssets()
  }, [])

  const [assetName, setAssetCode] = useState('')
  const selectedAsset = assets.find(asset => asset.name === assetName)
  const [bankName, setBankName] = useState('')
  const [publicKey, setPublicKey] = useState('')

  const resetFields = () => {
    setAssetCode('')
    setBankName('')
    setPublicKey('')
  }

  const createHoldingAccount = async e => {
    e.preventDefault()
    try {
      const accountId = await invoke('create_bank', {
        code: assetName,
        name: bankName,
        public_key: publicKey,
      })
      toast.success('Bank created')
      resetFields()
      if (!pathname.includes(routes.ACCOUNTS_PAGE_ROUTE) && redirectUponSubmission) {
        navigate(`${routes.ACCOUNTS_PAGE_ROUTE}/${accountId}`)
      } else {
        getBanks()
      }
    } catch (e) {
      toast.error(e.toString())
    }
  }

  return (
    <Card title={'Create Bank'} iconComponent={<IconBookOpen color={PRIMARY_COLOR} />}>
      <form onSubmit={createHoldingAccount} className={styles.formWrapper}>
        <div className={styles.flexDiv}>
          <label>
            <div>{'Asset'}</div>
            <select
              value={assetName}
              onChange={e => setAssetCode(e.target.value)}
              name={'asset-code'}
              className={styles.textInput}
            >
              <option />
              {assets.map((asset, i) => (
                <option value={asset.name} key={i}>{asset.name}</option>
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
            <div>{'Name'}</div>
            <input
              value={bankName}
              onChange={e => setBankName(e.target.value)}
              type={'text'}
              name={'bank-name'}
              className={styles.textInput}
            />
          </label>
        </div>
        <div>
          <label>
            <div>{'Public Key (optional)'}</div>
            <input
              value={publicKey}
              onChange={e => setPublicKey(e.target.value)}
              type={'text'}
              name={'public-key'}
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

const BanksTableSection = ({ banks, windowWidth }) => {
  return (
    <TableAccounts
      accounts={banks}
      isLoading={false}
      tableName={'Banks'}
      windowWidth={windowWidth}
      noPagination
    />
  )
}

const OnboardBankPage = ({ windowWidth }) => {
  const [banks, setBanks] = useState([])
  async function getBanks() {
    setBanks(await invoke('get_banks'))
  }

  useEffect(() => {
    getBanks()
  }, [])

  return (
    <Page
      withGlobalNav
      windowWidth={windowWidth}
      includeBackButton
    >
      <CreateBankCard getBanks={getBanks} />
      <BanksTableSection banks={banks} />
    </Page>
  )
}

export default OnboardBankPage
