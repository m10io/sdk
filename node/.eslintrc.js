module.exports = {
  extends: ['standard'],
  plugins: ['standard'],
  rules: {
    'no-var': 'error', // optional, recommended when using es6+
    'no-unused-vars': 1, // recommended
    'arrow-spacing': ['error', { before: true, after: true }], // recommended
    'comma-dangle': 0,

    // standard.js
    // do not require space before parenthesis when writing functions
    'space-before-function-paren': [
      'error',
      {
        named: 'never',
        anonymous: 'never',
        asyncArrow: 'never',
      },
    ],

    'array-bracket-spacing': ['error', 'never'],
    'space-infix-ops': ['error'],

    'generator-star-spacing': ['error', { before: false, after: true }],
    'max-len': ['error', {
      ignoreTemplateLiterals: true,
      ignoreUrls: true,
      ignoreTrailingComments: true,
      ignoreComments: true,
      code: 140,
    }],

    'prefer-destructuring': ['error', { object: false, array: false }],

    'no-tabs': 0,
    indent: 0,
    'no-multi-spaces': 0,

    // options to emulate prettier setup
    semi: ['error', 'never'],
    'template-curly-spacing': ['error', 'never'],
    'arrow-parens': ['error', 'as-needed'],

    // standard plugin - options
    'standard/object-curly-even-spacing': ['error', 'either'],
    'standard/computed-property-even-spacing': ['error', 'even'],
    'standard/no-callback-literal': ['error', ['cb', 'callback']],
  },
  parser: 'babel-eslint',
  parserOptions: {
    ecmaVersion: 8, // optional, recommended 6+
  },
}
