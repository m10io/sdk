import React from 'react'
import PropTypes from 'prop-types'
import classnames from 'classnames'
import styles from '../styles/button.module.scss'

export const BUTTON_THEME_PRIMARY = 'BUTTON_THEME_PRIMARY'
export const BUTTON_THEME_SECONDARY = 'BUTTON_THEME_SECONDARY'
export const BUTTON_THEME_RED_PRIMARY = 'BUTTON_THEME_RED_PRIMARY'

const BUTTON_THEME_TO_CLASSNAME_MAPPING = {
  [BUTTON_THEME_PRIMARY]: styles.buttonThemePrimary,
  [BUTTON_THEME_SECONDARY]: styles.buttonThemeSecondary,
  [BUTTON_THEME_RED_PRIMARY]: styles.buttonThemeRedPrimary,
}

const BUTTON_THEME_TO_BG_COLOR_MAPPING = {
  [BUTTON_THEME_PRIMARY]: '#00254d',
  [BUTTON_THEME_SECONDARY]: '#003166',
}

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
