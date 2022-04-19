import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import Page from '../components/page'
import Card from '../components/card'
import toast from 'react-hot-toast'
import Button, {
  BUTTON_THEME_PRIMARY,
} from '../components/button'
import TableAssets from '../components/table-assets'
import { TABLE_HEADER_THEME_CARD } from '../components/table'
import IconDollarSign from '../assets/icons/icon-dollar-sign'
import { PRIMARY_COLOR } from '../consts'
import styles from '../styles/shared.module.scss'

const CreateAssetCard = ({ reloadAssetsList }) => {
  const [value, setValue] = useState('')

  const createAsset = async e => {
    e.preventDefault()
    try {
      await invoke('create_asset', { code: value })
      toast.success('Asset created')
      setValue('')
      await reloadAssetsList()
    } catch (e) {
      toast.error(e.toString())
    }
  }

  return (
    <Card title={'Create Asset'} iconComponent={<IconDollarSign color={PRIMARY_COLOR} />}>
      <form onSubmit={createAsset} className={styles.formWrapper}>
        <label>
          <div>{'Name'}</div>
          <input
            value={value}
            onChange={e => setValue(e.target.value)}
            type={'text'}
            name={'createAsset'}
            className={styles.textInput}
          />
        </label>
        <Button theme={BUTTON_THEME_PRIMARY} type={'submit'} text={'Submit'} />
      </form>
    </Card>
  )
}

const AssetsPage = ({ windowWidth }) => {
  const [assets, updateAssets] = useState([])

  async function getAssets() {
    updateAssets(await invoke('get_assets'))
  }

  useEffect(() => {
    getAssets()
  }, [])

  return (
    <Page
      withGlobalNav
      windowWidth={windowWidth}
      includeBackButton
    >
      <TableAssets
        assets={assets}
        noPagination
        headerTheme={TABLE_HEADER_THEME_CARD}
      />
      <CreateAssetCard reloadAssetsList={getAssets} />
    </Page>
  )
}

export default AssetsPage
