# M10 Generic Bank WebApp

This adds a generic bank webapp frontend to pair with the generic bank backend

##### Current Pinned Versions:
- `node v16.14.2`
- `npm  v6.14.4`


## Developer quickstart

- create `.env` file with following keys:
  ```
  NODE_ENV=development
  OAUTH_DOMAIN=https://keycloak-app.m10.net
  OAUTH_AUDIENCE=https://api.m10.net

  OXIDE_API_URL=https://develop.m10.net/m10-bank/api/v1/
  BASE_API_URL=https://develop.m10.net/

  BANK_NAME=Omega Bank
  BANK_ASSET=USD
  BANK_PRIMARY_COLOR=#4b0908
  ```
- run `yarn install`
- run `yarn dev`

## Authentication
Authentication is handled via [Auth0](https://auth0.com). Rather than support require a Client ID, we have set up a separate authentication service with two notable routes, both found in `/pages/api/auth`:

1. Login: `${process.env.BASE_API_URL}/oauth/token`
  - This route returns a Bearer Token to be used in all subsequent requests in the Authorization header.

2. Get User: `${process.env.OAUTH_DOMAIN}/realms/master/protocol/openid-connect/userinfo`
  - This is a GET request that returns user info. All that is required is a Bearer token in the Authorization header.

The `BASE_API_URL` env variable is used exclusively for this separate authentication service.

Additionally, we have included user-specific metadata, such as `name` and `phone` in a `attributes` field in the return user object JWT. This field name can be changed according to auth0 specifications.

Finally, Auth0 allows creating Roles in order to subdivide permissions for authenticated users. In order to view the admin pages of this webapp, one of the following roles are required:

- `omega-admin`
- `omega-test-admin`

The aforementioned user object includes a `resource_access` field that includes user roles by client. The implementation of this can be found in `utils/auth`.

## Directory structure

- **/assets**
  - static file assets including images and SVGs
- **/components**
  - reusable React components (some of them are coupled to certain features and used just in one place, but this should be limited)
  - **/styles**
    - contains all `component` styles
- **/localization**
  - regional/language copy alternatives
- **/pages**
  - non-reusable React components representing Page routes. Each Page's filename will correspond to its page route. Examples:
    - `banks.js` -> `/banks`
    - `/banks/index.js` -> `/banks`
    - `/banks/[id].js` -> `/banks/<id>` (id will be passed to Page component as a prop)
    - **/api**
      - auto-generated api routes. These pages return JSON rather than HTML/CSS/JS
- **/public**
  - publicly available files such as favicon and `robots.txt`
- **/utils**
  - various utility functions usable throughout the app

## Coding style

ESLint rules are configured in the `.eslintrc.js` file and extend from the base `eslint-config-standard` config.
Documentation for any rule may be found via googling `eslint <rule-name>`.
List of all basic rules: http://eslint.org/docs/rules/

#### ESLint via command line

```
yarn lint
```

## Debugging
1. Start the app in debug mode - `yarn debug`
2. Open Chrome and go to `chrome://inspect`. In the "Remote Target" section, click "inspect" next to the listed corresponding application.
3.  Add a `.vscode/launch.json` file to the root `bank-webapp` folder:
```
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "node",
      "request": "attach",
      "name": "Launch Program",
      "skipFiles": ["<node_internals>/**"],
      "port": 9229
    }
  ]
}
```
4. In VS Code, open Command Palette and start the debugger (F5)
5. Add `debugger` statements throughout the frontend or backend code
6. Run the app in browser, stopping at debugger breakpoints

See the [official docs](https://nextjs.org/docs/advanced-features/debugging) for complete instructions.

## Production mode during development

It's useful to test production build to ensure that pages are correctly server-side rendered and that configurations are handled as intended.

Build locally in production mode:
```
rm -rf node_modules
yarn install
yarn build
yarn start
```
