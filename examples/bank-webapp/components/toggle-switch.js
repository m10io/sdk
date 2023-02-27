import classnames from 'classnames'
import PropTypes from 'prop-types'
import styles from './styles/toggle-switch.module.scss'

const ToggleSwitch = ({
  className,
  label,
  onChange = () => {},
  checked = false,
  labelClassName,
}) => {
  return (
    <div className={classnames(
      styles.container,
      className,
    )}>
      {label && (
        <div className={classnames(
          styles.textLabel,
          labelClassName,
        )}>
          {`${label} `}
        </div>
      )}
      <div className={styles.toggleSwitch}>
        <input
          type={'checkbox'}
          className={styles.checkbox}
          name={label}
          id={label}
          checked={checked}
          onChange={onChange}
        />
        <label className={styles.label} htmlFor={label}>
          <span className={styles.inner} />
          <span className={styles.switch} />
        </label>
      </div>
    </div>
  )
}

ToggleSwitch.propTypes = {
  className: PropTypes.string,
  labelClassName: PropTypes.string,
  label: PropTypes.string,
  onChange: PropTypes.func.isRequired,
  checked: PropTypes.bool.isRequired,
}

export default ToggleSwitch
