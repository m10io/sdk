import React from 'react'
import {
  BrowserRouter as Router,
  Routes,
  Route,
} from 'react-router-dom'
import { Toaster } from 'react-hot-toast'
import { GlobalProvider } from './provider'
import SplashPage from './pages/splash-page'
import AssetsPage from './pages/assets-page'
import AccountsPage from './pages/accounts-page'
import LedgerInfoPage from './pages/ledger-info-page'
import IssuePage from './pages/issue-page'
import RedeemPage from './pages/redeem-page'
import OnboardBankPage from './pages/onboard-bank-page'
import routes from './routes'
import '@fontsource/poppins'
require('typeface-rubik')
require('typeface-libre-baskerville')

export default function App() {
  return (
    <GlobalProvider>
      <Router>
        <div>
          <Toaster position={'top-right'} />
          <Routes>
            <Route path={routes.BASE_ROUTE} element={<SplashPage />} />
            <Route path={routes.ASSETS_PAGE_ROUTE} element={<AssetsPage />} />
            <Route path={routes.LEDGER_INFO_PAGE_ROUTE} element={<LedgerInfoPage />} />
            <Route path={routes.ISSUE_PAGE_ROUTE} element={<IssuePage />} />
            <Route path={routes.REDEEM_PAGE_ROUTE} element={<RedeemPage />} />
            <Route path={routes.ONBOARD_BANK_PAGE_ROUTE} element={<OnboardBankPage />} />
            <Route path={`${routes.ACCOUNTS_PAGE_ROUTE}/:accountId`} element={<AccountsPage />} />
          </Routes>
        </div>
      </Router>
    </GlobalProvider>
  );
}
