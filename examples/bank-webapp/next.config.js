const path = require('path')
const dotenv = require('dotenv')
dotenv.config()

const basePath = ''

const nextConfig = {
  basePath: basePath,
  webpack: config => {
    config.resolve.modules.push(path.resolve('./'))
    config.resolve.modules.push(path.resolve('./components'))
    config.resolve.modules.push(path.resolve('./pages'))
    config.resolve.modules.push(path.resolve('./components/styles'))
    config.resolve.modules.push(path.resolve('./assets'))
    config.resolve.modules.push(path.resolve('./lib'))
    return config
  },
  // NOTE: client-side keys must be included here
  publicRuntimeConfig: {
    apiBaseUrl: '/api/v1',
    authPublicDomain: process.env.OAUTH_DOMAIN,
    authPublicClientId: process.env.OAUTH_CLIENT_ID,
    authPublicAudience: process.env.OAUTH_AUDIENCE,
    stripeSecret: process.env.STRIPE_SECRET,
    nodeEnv: process.env.NODE_ENV,
    bankName: process.env.BANK_NAME,
    bankAsset: process.env.BANK_ASSET,
    bankLogoUrl: process.env.BANK_LOGO_URL,
    bankPrimaryColor: process.env.BANK_PRIMARY_COLOR,
    bankSecondaryColor: process.env.BANK_SECONDARY_COLOR,
  },
  images: {
    domains: [
      's.gravatar.com', // auth0 avatar PNG CDN
      'm10.net',
      'qa.m10.net',
      'dev.m10.net',
      'test.m10.net',
    ],
    path: `${basePath}/_next/image`,
  },
  sassOptions: {
    prependData: `
      $bankPrimaryColor: ${process.env.BANK_PRIMARY_COLOR};
      $bankSecondaryColor: ${process.env.BANK_SECONDARY_COLOR};
    `,
  },
}

module.exports = nextConfig
