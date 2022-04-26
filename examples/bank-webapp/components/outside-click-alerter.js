import React, { Component } from 'react'
import PropTypes from 'prop-types'

/**
 * Component that alerts if you click outside of it.
 *
 * @param   {node}      children            The node whose area is tracked to determine if an outside click has been done.
 * @param   {function}  onOutsideClick      Function that is called if click is made outside of component's area.
 * @param   {boolean}   shouldTrackClick    Whether outside clicks should trigger the 'onOutsideClick' prop.
 * @param   {array}     classNamesToIgnore  Array of strings of classnames that can be clicked on without triggering 'onOutsideClick' action
 * @param   {array}     elementIdsToIgnore    Array of strings of id's that can be clicked on without triggering 'onOutsideClick' action
 * @return  {node}                          A component that alerts if you click outside of it.
 */
class OutsideClickAlerter extends Component {
  constructor(props) {
    super(props)
    this.isEventListenerAdded = false
  }

  componentDidMount() {
    const { shouldTrackClick } = this.props
    if (shouldTrackClick) {
      document.addEventListener('mousedown', this.handleClickOutside)
      this.isEventListenerAdded = true
    }
  }

  componentWillUnmount() {
    document.removeEventListener('mousedown', this.handleClickOutside)
  }

  UNSAFE_componentWillReceiveProps(nextProps) { // eslint-disable-line
    const { shouldTrackClick } = nextProps

    if (shouldTrackClick && !this.isEventListenerAdded) {
      document.addEventListener('mousedown', this.handleClickOutside)
      this.isEventListenerAdded = true
    }
    if (!shouldTrackClick && this.isEventListenerAdded) {
      document.removeEventListener('mousedown', this.handleClickOutside)
      this.isEventListenerAdded = false
    }
  }

  setWrapperRef = el => {
    this.wrapperRef = el
  }

  handleClickOutside = event => {
    const {
      onOutsideClick,
      // classNamesToIgnore,
      elementIdsToIgnore,
    } = this.props

    // NOTE: NextJS is changing classNames during build, so commenting this out until we can resolve that.
    // const clickedOnClassName = event.target.className
    // const shouldIgnoreClickedElementByClassName = (
    //   (classNamesToIgnore || []).some((classNameToIgnore) => {
    //     return ((clickedOnClassName || '').includes(classNameToIgnore))
    // }))

    const clickedOnClassId = event.target.id
    const shouldIgnoreClickedElementById = (
      (elementIdsToIgnore || []).some(classIdToIgnore => (
        (clickedOnClassId || '').includes(classIdToIgnore)
      ))
    )

    const shouldIgnoreClickedElement =
      // shouldIgnoreClickedElementByClassName ||
      shouldIgnoreClickedElementById

    if (
      this.isEventListenerAdded &&
      !shouldIgnoreClickedElement &&
      this.wrapperRef &&
      !this.wrapperRef.contains(event.target)
    ) {
      onOutsideClick()
    }
  }

  render() {
    const { children, className } = this.props
    return (
      <div className={className} ref={this.setWrapperRef}>
        {children}
      </div>
    )
  }
}

OutsideClickAlerter.propTypes = {
  children: PropTypes.node.isRequired,
  onOutsideClick: PropTypes.func.isRequired,
  shouldTrackClick: PropTypes.bool.isRequired,
  classNamesToIgnore: PropTypes.arrayOf(PropTypes.string),
  elementIdsToIgnore: PropTypes.arrayOf(PropTypes.string),
  className: PropTypes.string,
}

export default OutsideClickAlerter
