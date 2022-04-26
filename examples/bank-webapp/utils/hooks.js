import { useEffect, useState } from 'react'
import { useRouter } from 'next/router'
import axios from 'axios'
import Cookies from 'js-cookie'

export const useApi = (url, options = {}, conditional = true) => {
  const [state, setState] = useState({
    error: null,
    isLoading: true,
    data: null,
    isLoaded: false,
  })
  const [refreshIndex, setRefreshIndex] = useState(0)

  // prevent call if no conditonal data, such a required id from a separate get request
  if (!conditional) {
      return {
      ...state,
      refresh: updatedOptions => setRefreshIndex(refreshIndex + 1),
    }
  }

  useEffect(() => {
    (async() => {
      try {
        const res = await axios.get(url, {
          params: options,
          headers: {
            ...options.headers,
            // Add the Authorization header to the existing headers
            Authorization: `Bearer ${Cookies.get('access_token')}`,
          },
        })
        setState({
          ...state,
          data: res?.data,
          error: null,
          isLoading: false,
          isLoaded: true,
        })
      } catch (error) {
        setState({
          ...state,
          error,
          isLoading: false,
        })
      }
    })()
  }, [refreshIndex])

  return {
    ...state,
    refresh: () => {
      setRefreshIndex(refreshIndex + 1)
    },
  }
}

/** Returns the page query, or null if the page is not yet hydrated. */
export function useQuery() {
  const router = useRouter()

  const hasQueryParams =
    /\[.+\]/.test(router.route) || /\?./.test(router.asPath)
  const ready = !hasQueryParams || Object.keys(router.query).length > 0

  if (!ready) return null

  return router.query || {}
}
