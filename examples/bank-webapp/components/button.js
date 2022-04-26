import React from 'react'
import PropTypes from 'prop-types'
import Image from 'next/image'
import LoadingSpinnerIcon from 'assets/icons/loading-spinner.svg'
import classnames from 'classnames'
import styles from './styles/button.module.scss'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

export const BUTTON_THEME_PRIMARY = 'BUTTON_THEME_PRIMARY'
export const BUTTON_THEME_SECONDARY = 'BUTTON_THEME_SECONDARY'
export const BUTTON_THEME_RED_PRIMARY = 'BUTTON_THEME_RED_PRIMARY'

const BUTTON_THEME_TO_CLASSNAME_MAPPING = {
  [BUTTON_THEME_PRIMARY]: styles.buttonThemePrimary,
  [BUTTON_THEME_SECONDARY]: styles.buttonThemeSecondary,
  [BUTTON_THEME_RED_PRIMARY]: styles.buttonThemeRedPrimary,
}

const BUTTON_THEME_TO_BG_COLOR_MAPPING = {
  [BUTTON_THEME_PRIMARY]: publicRuntimeConfig.bankPrimaryColor,
  [BUTTON_THEME_SECONDARY]: publicRuntimeConfig.bankPrimaryColor,
}

export const LoadingSpinner = (
  <Image
    src={LoadingSpinnerIcon}
    className={styles.loadingSpinner}
    width={25}
    height={25}
    alt={'loading'}
  />
)

const Button = ({
  text,
  onClick,
  theme,
  children,
  type,
  icon,
  className,
  style,
  id,
  disabled = false,
  isLoading,
  isSuccessful,
}) => {
  return (
    <button
      onClick={disabled ? () => {} : onClick}
      role={'button'}
      tabIndex={'0'}
      className={classnames(
        styles.buttonWrapper,
        BUTTON_THEME_TO_CLASSNAME_MAPPING[theme],
        disabled && styles.buttonDisabled,
        className,
      )}
      style={{
        ...style,
        background: BUTTON_THEME_TO_BG_COLOR_MAPPING[theme],
      }}
      type={type || 'button'}
      id={id}
    >
      {children || text}
    </button>
  )
}

export const ButtonDynamic = ({
  isLoading,
  isSuccessful,
  isFailed,
  onSuccess = () => {},
  className,
  children,
  text,
  style,
  theme,
  ...props
}) => {
  if (isSuccessful) setTimeout(() => (onSuccess()), 500)
  const DynamicButtonText = ({ hidden }) => (
    <div className={classnames(
      hidden && styles.hidden,
      styles.dynamicButtonText,
      isSuccessful && styles.hidden,
      isFailed && styles.hidden,
    )}>
      {children || text}
    </div>
  )
  return (
    <Button
      className={classnames(
        styles.buttonDynamicWrapper,
        className,
      )}
      style={style}
      theme={theme}
      {...props}
    >
      <div className={classnames(
        styles.dynamicButtonContent,
        isSuccessful && styles.success,
        isFailed && styles.fail,
      )}>
        <DynamicButtonText hidden={isLoading} />
        {isLoading && (
          <div className={styles.dynamicButtonLoadingSpinner}>
            {LoadingSpinner}
          </div>
        )}
      </div>
    </Button>
  )
}

Button.propTypes = {
  text: PropTypes.string,
  // if Button component is wrapped by button form component, it doesn't require onClick
  onClick: PropTypes.func,
  theme: PropTypes.string,
  // allow 'children' for customizing buttons, e.g. with an SVG
  children: PropTypes.node,
  type: PropTypes.string,
  className: PropTypes.string,
  id: PropTypes.string,
  icon: PropTypes.string,
  style: PropTypes.object,
  disabled: PropTypes.bool,
}

Button.defaultProps = {
  theme: BUTTON_THEME_PRIMARY,
}

export default Button
