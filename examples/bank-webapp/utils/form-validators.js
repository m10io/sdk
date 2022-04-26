export const composeValidators = (...validators) => value => (
  validators.reduce((error, validator) => error ||
  validator(value), undefined)
)

export const isRequiredValidator = value => (value ? undefined : 'Required')
