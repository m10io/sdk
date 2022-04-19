import React, { createContext, useReducer } from 'react'
import AppReducer from './reducer'
// import { invoke } from '@tauri-apps/api/tauri'

const initialState = {
   assets: [],
   banks: [],
}

export const GlobalContext = createContext(initialState)

export const GlobalProvider = ({ children }) => {
   const [state, dispatch] = useReducer(AppReducer, initialState)

   function addAsset(asset) {
     dispatch({
       type: 'ADD_ASSET',
       payload: asset
     })
   }

   function addBank(bank) {
     dispatch({
       type: 'ADD_BANK',
       payload: bank
     })
   }

   return(
    <GlobalContext.Provider value={{
      assets: state.assets,
      addAsset,
      banks: state.banks,
      addBank,
    }}>
      {children}
    </GlobalContext.Provider>
  )
}
