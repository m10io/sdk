/* eslint-disable camelcase */
import React, { useState } from 'react'
import classnames from 'classnames'
import PropTypes from 'prop-types'
import Router from 'next/router'
import Cookies from 'js-cookie'
import { Form } from 'react-final-form'
import FormField, {
  FORM_FIELD_THEME_AUTHENTICATION,
  FORM_FIELD_PASSWORD,
  FORM_FIELD_TEXT,
} from 'components/form-field'
import {
  isRequiredValidator,
} from 'utils/form-validators'
import {
  determineHomeRouteByUser,
  parseJwt,
} from 'utils/auth'
import {
  ButtonDynamic,
  BUTTON_THEME_SECONDARY,
} from 'components/button'
import { login } from 'lib/api/auth'
import styles from 'components/styles/login.module.scss'
import getConfig from 'next/config'
const { publicRuntimeConfig } = getConfig()

const LoginForm = ({ isSmallMobileViewport }) => {
  const [loginState, setLoginState] = useState({
    error: null,
    isSubmitting: false,
    isSubmitted: false,
    user: null,
  })

  const onClickLogin = async body => {
    setLoginState({ isSubmitting: true, error: null, isSubmitted: false, user: null })
    try {
      const res = await login(body)
      // get tokens
      const user = parseJwt(res?.data?.id_token)
      setLoginState({ user, isSubmitted: true, isSubmitting: false, error: null })

      // set tokens as cookies for later use in api calls
      Cookies.set('access_token', res?.data?.access_token)
    } catch (e) {
      setLoginState({ user: null, isSubmitted: false, isSubmitting: false, error: e })
    }
  }
  return (
    <div className={classnames(
      styles.loginCardWrapper,
      isSmallMobileViewport && styles.loginCardWrapperSmallMobileViewport,
    )}>
      {publicRuntimeConfig.bankLogoSrc
        ? (
          <img src={publicRuntimeConfig.bankLogoSrc} style={{ width: 200 }} alt={publicRuntimeConfig.bankName} />
        ) : (
          <div className={styles.bankLogoName} style={{ background: publicRuntimeConfig.bankPrimaryColor }}>
            {publicRuntimeConfig.bankName}
          </div>
        )}
      <div className={styles.welcomeText}>
        {'Welcome back'}
      </div>
      <div className={styles.instructionsText}>
        {'Enter your credentials to access your account.'}
      </div>
      <Form
        onSubmit={onClickLogin}
        validate={values => {
          const errors = {}
          return errors
        }}
        render={renderValues => {
          const { handleSubmit, form } = renderValues
          return (
            <form className={styles.loginFormWrapper} onSubmit={handleSubmit}>
              <FormField
                fieldName={'username'}
                label={'Email'}
                type={FORM_FIELD_TEXT}
                theme={FORM_FIELD_THEME_AUTHENTICATION}
                className={styles.singleFormField}
                validators={[isRequiredValidator]}
                onClick={() => setLoginState({ error: null, ...loginState })}
              />
              <FormField
                fieldName={'password'}
                label={'Password'}
                type={FORM_FIELD_PASSWORD}
                theme={FORM_FIELD_THEME_AUTHENTICATION}
                className={styles.singleFormField}
                validators={[isRequiredValidator]}
              />
              <ButtonDynamic
                theme={BUTTON_THEME_SECONDARY}
                text={'Submit'}
                type={'submit'}
                className={styles.submitButton}
                isLoading={loginState.isSubmitting}
                isSuccessful={loginState.isSubmitted}
                onSuccess={() => {
                  // determine redirect route by user permissions / type
                  const redirectTo = Router?.router?.query?.redirect || determineHomeRouteByUser(loginState.user)
                  Router.replace(redirectTo)
                }}
              />
              {loginState.error && (
                <div className={styles.loginFormErrorText}>
                  {'Incorrect username or password'}
                </div>
              )}
            </form>
          )
        }}
      />
    </div>
  )
}

const LoginPage = ({
  windowWidth,
}) => {
  const isSmallViewport = true
  // NOTE: Pixel XL 2 is 411 pixels wide, iPhone X is 375
  const isSmallMobileViewport = windowWidth < 412
  return (windowWidth &&
    <div className={classnames(
      styles.loginPageWrapper,
      isSmallViewport && styles.loginPageWrapperMobile,
    )}>
      <div className={classnames(
        styles.loginCardFormWrapper,
        isSmallMobileViewport && styles.loginCardFormWrapperSmallMobileViewport,
      )}>
        <div className={styles.loginCardContent}>
          <LoginForm isSmallMobileViewport={isSmallMobileViewport} />
        </div>
        <div className={classnames(
          styles.infoWrapper,
          isSmallViewport && styles.infoWrapperMobile,
        )}>
          {/*
          <div className={styles.needAccessText}>
            {'Need access? '}
            <a href={'mailto:email@email.com'}>{'Contact an admin'}</a>
          </div>
        */}
          <div className={styles.copyrightText}>
            {'© 2022, M10 Networks, Inc. All rights reserved'}
          </div>
        </div>
      </div>
    </div>
  )
}

LoginPage.propTypes = {
  windowWidth: PropTypes.number,
  query: PropTypes.object,
  loginError: PropTypes.object,
}

export default LoginPage
