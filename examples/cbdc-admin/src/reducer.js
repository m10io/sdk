const reducer = (state, action) => {
   switch(action.type) {
     // assets
     case 'ADD_ASSET':
       return {
         assets: [...state.assets, action.payload]
       }
     case 'LIST_ASSETS':
       return {
         assets: action.payload,
       }

      // banks
     case 'LIST_BANKS':
       return {
         banks: action.payload,
       }
     case 'ADD_BANK':
       return {
         banks: [...state.banks, action.payload]
       }
     default:
       return state;
   }
}

export default reducer
