# FontFactory

Created with `create-near-app`

## Contract

Build and deploy your contract to TestNet with a temporary dev account:
`npm run deploy`

Test your contract:
`npm test`

Run the frontend
`npm start`

## Structure

1. Smart-contract code is in the `/contract` folder.
    - See the README there for more info.
    - In blockchain apps the smart contract is the "backend" of your app.
2. The frontend connected to the blockchain is in the `/frontend` directory.
    - `/frontend/index.html` is a great place to start exploring.
    - Note that it loads in `/frontend/index.js`
3. Test your contract with `npm test`
    - This run the tests in `integration-tests` directory.

## Deploy

Every smart contract in NEAR has its **own associated account.**
Run `npm run deploy` to deploye your contract to the live NEAR TestNet with a temporary dev account.
Permanent deployment:

1.  Use near-cli globally `npm install --g near-cli` or inside the project, check which version is installed `near --version` (or `npx near --version`)
2.  Create an account for the contract: `near-blank-project.your-name.testnet`

    -   `near login`
    -   Create a subaccount (replace `YOUR-NAME` below with your actual account name):
        ```Bash
        near create-account near-blank-project.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet
        ```

3.  Deploy the contract

    -   Use the CLI to deploy the contract to TestNet with your account ID.
    -   Replace `PATH_TO_WASM_FILE` with the `wasm` that was generated in `contract` build directory.
        ```Bash
        near deploy --accountId near-blank-project.YOUR-NAME.testnet --wasmFile PATH_TO_WASM_FILE
        ```

4.  Set contract name in your frontend code

    -   Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.
    -   const CONTRACT_NAME = process.env.CONTRACT_NAME || 'near-blank-project.YOUR-NAME.testnet'

## Reference

-   [create-near-app](https://github.com/near/create-near-app)
-   [Node.js](https://nodejs.org/en/download/package-manager/)
-   [jest](http(//jestjs.io/)
-   [NEAR accounts](https://docs.near.org/concepts/basics/account)
-   [NEAR Wallet](https://wallet.testnet.near.org/)
-   [near-cli](https://github.com/near/near-cli)
-   [gh-pages](https://github.com/tschaub/gh-pages)
