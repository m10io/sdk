/* eslint-disable react/no-danger */
import { useState, useEffect } from 'react'
import Head from 'next/head'
import getConfig from 'next/config'
import lodashGet from 'lodash/get'
import 'styles/base.scss'
const { publicRuntimeConfig } = getConfig()

const MyApp = ({ pageProps, Component }) => {
  // NOTE: initialize with desktop viewport width
  const [windowWidth, setWindowWidth] = useState(null)
  const [windowHeight, setWindowHeight] = useState(null)

  const handleResize = () => {
    setWindowWidth(lodashGet(document, 'body.clientWidth'))
    setWindowHeight(lodashGet(window, 'innerHeight'))
  }

  const setWindowResizeHandler = () => {
    window.addEventListener('resize', handleResize)
    setTimeout(handleResize, 50)
  }

  useEffect(() => {
    setWindowResizeHandler()
  }, [])

  return (
    <>
      <Head>
        <title>{`${publicRuntimeConfig.bankName}${publicRuntimeConfig.nodeEnv === 'development' ? ' - dev' : ''}`}</title>
        <link
          href={'https://fonts.googleapis.com/css2?family=Rubik:wght@400;500&display=swap'}
          rel={'stylesheet'}
        />
        <link
          href={'https://fonts.googleapis.com/css2?family=Libre+Baskerville&display=swap'}
          rel={'stylesheet'}
        />
        {/*
          <link rel={'icon'} href={'/logo.svg'} />
          <link rel={'mask-icon'} href={'/logo.svg'} color={'#384764'} />
          <link rel={'apple-touch-icon'} href={'/logo.png'} />
        */}
      </Head>
      <Component
        {...pageProps}
        windowWidth={windowWidth}
        windowHeight={windowHeight}
      />
    </>
  )
}

export default MyApp
