export const useQuery = () => {
  return window.location.search
    ? new URLSearchParams(window.location.search)
    : { get: () => {} }
}
