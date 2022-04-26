import React from 'react'
import PropTypes from 'prop-types'
import classnames from 'classnames'
import { Field } from 'react-final-form'
import styles from './styles/form-field.module.scss'

export const FORM_FIELD_THEME_AUTHENTICATION = 'FORM_FIELD_THEME_AUTHENTICATION'

const FORM_FIELD_THEME_TO_CLASSNAME_MAPPING = {
  [FORM_FIELD_THEME_AUTHENTICATION]: styles.formFieldThemeAuthentication,
}

// form field types
export const FORM_FIELD_TEXT = 'text'
export const FORM_FIELD_PASSWORD = 'password'
// utils
const composeValidators = funcsArr => value =>
  funcsArr.reduce((error, validator) => error || validator(value), undefined)

const Error = ({ name, errorClassName, forceValidate }) => (
  <Field
    name={name}
    subscription={{ touched: true, error: true }}
    render={({ meta: { touched, error } }) => {
      return (
        (touched && error || forceValidate) // eslint-disable-line
          ? <div className={errorClassName}>{error}</div>
          : null
      )
    }}
  />
)

const determineTextFieldType = type => {
  switch (type) {
    case 'number':
      return 'number'
    case 'password':
      return 'password'
    default:
      return 'text'
  }
}

const FormFieldText = ({
  fieldName,
  label,
  type,
  labelClassName,
  errorClassName,
  placeholder,
  className,
  validate,
  wrapperClassName,
  theme,
  value,
}) => {
  return (
    <Field
      name={fieldName}
      type={'text'}
      validate={validate}
    >
      {({ input, meta }) => {
        return (
          <div className={classnames(
            FORM_FIELD_THEME_TO_CLASSNAME_MAPPING[theme],
            className,
          )}>
            <label className={labelClassName} htmlFor={fieldName}>{label}</label>
            <div className={styles.textFormFieldInputWrapper}>
              <input
                {...input}
                id={fieldName}
                type={determineTextFieldType(type)}
                placeholder={placeholder}
                autoComplete={'true'}
                value={value}
                onChange={e => input.onChange(e)}
              />
            </div>
            <Error
              name={fieldName}
              errorClassName={classnames(
                FORM_FIELD_THEME_TO_CLASSNAME_MAPPING[theme],
                styles.error,
                errorClassName,
              )}
            />
          </div>
        )
      }}
    </Field>
  )
}

const FormField = ({
  type,
  fieldName,
  label,
  labelClassName,
  errorClassName,
  selectArrowClassName,
  options = [],
  placeholder,
  className,
  isRequired,
  singleCheckboxFieldName,
  singleCheckboxText,
  theme,
  validators = [],
  value,
  setValue,
  preventRender,
}) => {
  if (preventRender) return null
  const composedValidators = composeValidators([...validators])
  switch (type) {
    case FORM_FIELD_TEXT:
    case FORM_FIELD_PASSWORD:
      return (
        <FormFieldText
          type={type}
          fieldName={fieldName}
          label={label}
          labelClassName={labelClassName}
          placeholder={placeholder}
          errorClassName={errorClassName}
          className={className}
          validate={composedValidators}
          theme={theme}
          value={value}
        />
      )
    default:
      return null
  }
}

FormField.propTypes = {
  type: PropTypes.oneOf([
    FORM_FIELD_TEXT,
    FORM_FIELD_PASSWORD,
  ]).isRequired,
  fieldName: PropTypes.string.isRequired,
  label: PropTypes.string,
  labelClassName: PropTypes.string,
  errorClassName: PropTypes.string,
  options: PropTypes.arrayOf(PropTypes.shape({
    name: PropTypes.string.isRequired,
    value: PropTypes.string.isRequired,
  })),
  placeholder: PropTypes.string,
  className: PropTypes.string,
  isRequired: PropTypes.bool,
  singleCheckboxFieldName: PropTypes.string,
  singleCheckboxText: PropTypes.string,
  theme: PropTypes.oneOf(Object.keys(FORM_FIELD_THEME_TO_CLASSNAME_MAPPING)),
  validators: PropTypes.arrayOf(PropTypes.func.isRequired),
  value: PropTypes.string,
  setValue: PropTypes.func,
  preventRender: PropTypes.bool,
  selectArrowClassName: PropTypes.string,
}

export default FormField
