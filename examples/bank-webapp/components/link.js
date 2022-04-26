import React from 'react'
import classnames from 'classnames'
// import NextJSLink from 'next/link'
import PropTypes from 'prop-types'
import styles from './styles/link.module.scss'

export const LINK_THEME_BLUE = 'LINK_THEME_BLUE'
export const LINK_THEME_GOLD = 'LINK_THEME_GOLD'

const LINK_THEMES_TO_CLASSNAME_MAPPING = {
  LINK_THEME_BLUE: styles.linkThemeBlue,
  LINK_THEME_GOLD: styles.linkThemeGold,
}

const Link = ({
  children,
  href,
  className,
  openInNewWindow,
  isEmail,
  theme,
  onClick,
  disabled,
  ...props
}) => {
  const linkClassName = classnames(
    LINK_THEMES_TO_CLASSNAME_MAPPING[theme],
    className,
  )

  // handle a link with a conditional null value 'href' prop
  if (!href) {
    return (
      <div
        className={classnames(
          linkClassName,
          disabled && styles.linkDisabled,
        )}
        onClick={onClick}
        role={'button'}
        tabIndex={0}
        {...props}
      >
        {children}
      </div>
    )
  }

  // TODO: determine via regex rather than 'isEmail' boolean
  if (isEmail) {
    return (
      <a
        href={`mailto:${href}`}
        className={className}
        {...props}
      >
        {children}
      </a>
    )
  }

  // // TODO: specific handling for file links
  // const file = /\.[0-9a-z]+$/i.test(href)
  // // assumes that any internal link begins with '/', anything else is external
  // const internal = /^\/(?!\/)/.test(href)

  const target = openInNewWindow ? '_blank' : ''

  // TODO: utilize NextJSLink
  // next/link docs: https://nextjs.org/docs/api-reference/next/link
  return (
    <a
      href={href}
      className={classnames(
        linkClassName,
        disabled && styles.linkDisabled,
      )}
      target={target}
      onClick={onClick}
      {...props}
    >
      {children}
    </a>
  )
}

Link.propTypes = {
  children: PropTypes.node,
  href: PropTypes.oneOfType([
    PropTypes.string,
    PropTypes.shape({ pathname: PropTypes.string, query: PropTypes.object }),
  ]),
  as: PropTypes.string,
  className: PropTypes.string,
  openInNewWindow: PropTypes.bool,
  isEmail: PropTypes.bool,
  theme: PropTypes.oneOf(Object.keys(LINK_THEMES_TO_CLASSNAME_MAPPING)),
  onClick: PropTypes.func,
}

export default Link
