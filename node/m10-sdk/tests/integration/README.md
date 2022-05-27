# Sample CBDC Recipe

This section is intended to inform the user 

## Terminology

The following is a list of actors with the roles they would play in this system. 

| Term      | Description |
| ----------- | ----------- |
| CBDC Admin  | The central banks operator that is responsible for issuing and destroying m0 currency. Responsibilities include creating accounts for M1 banks and transferrence as well.        |
| M1 Bank  |   A bank created on behalf of the M1 bank by the CBDC admin.    |
| Operator  |  The account used by the m0 currency issuer for CBDC operations. This has more elevated access than the CBDC key.       |
| Instrument | An m0 currency issued by the CBDC. The instrument in this example is 'XYZ'  |

# Recipe steps

The following steps are an in-order example of setting up an hybrid CBDC model.

# Recipe location

The integration test area can be [found here](./hybrid.test.ts)

## Creating m0 currency

Creating m0 currency for issuance requires creating an appropriate instrument first.

The test header is `Create m0 currency`.

## Destroy m0 currency

<TBD>

## Creating M1 Bank accounts

Creating m1 bank accounts requires creating an approrpiate m0 currency first.

The test header is `Creating m1 bank accounts for XYZ currency`.

## Issuing m0 currency to m1 bank

## Redemption of m1 currency to CBDC





