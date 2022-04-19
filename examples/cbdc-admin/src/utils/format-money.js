export default function formatDollarValue(value) {
  // NOTE: expects 'value' param in cents
  const valueInDollars = parseInt(value, 10) / 100
  if (valueInDollars === 0) return '0.00'
  return valueInDollars.toFixed(2).replace(/\d(?=(\d{3})+\.)/g, '$&,').replace(/\.00/g, '')
}
