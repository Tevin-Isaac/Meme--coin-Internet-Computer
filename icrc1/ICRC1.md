## ICRC1
ICRC1 is a token standard created by the Internet Computer working group. The acronym "ICRC" stands for "Internet Computer Request for Comments," and the ICRC1 standard is a standard for fungible tokens on the Internet Computer.

The ICRC1 standard defines a set of rules and functionalities for creating and recording token transactions on the Internet Computer. It specifies the general functionalities of ledgers, and any tokens and their corresponding ledgers that want to support the ICRC1 standard must fulfill all requirements within the standard.

The purpose of the ICRC1 standard is to create a universally accepted standard for fungible tokens on the Internet Computer, ensuring interoperability and compatibility across different token contracts and applications within the Internet Computer ecosystem.

By adhering to the ICRC1 standard, token contracts can ensure that they provide consistent and predictable behaviors for transferring tokens, querying token balances, and managing token allowances. This standardization allows fungible tokens to be seamlessly integrated into various decentralized applications (dApps) and smart contracts on the Internet Computer.

In summary, ICRC1 is a token standard that provides a common framework for creating fungible tokens on the Internet Computer, enabling developers to build and deploy token contracts that are compatible with a wide range of applications and services within the Internet Computer ecosystem.

## ICRC-1 ledger setup
* Step 1: Make sure you use a recent version of the IC SDK.

* Step 2: Create a new dfx project with the command:
```bash
dfx new icrc1_ledger_canister
cd icrc1_ledger_canister
```
* Step 3: Determine ledger file locations


OPTIONAL: If you want to make sure, you have the latest ICRC-1 ledger files you can run the following script:
```bash
curl -o download_latest_icrc1_ledger.sh "https://raw.githubusercontent.com/dfinity/ic/326df23607fc8280a047daba2d8462f1dfc57466/rs/rosetta-api/scripts/download_latest_icrc1_ledger.sh"

chmod +x download_latest_icrc1_ledger.sh

./download_latest_icrc1_ledger.sh

```

Step 4: Configure the dfx.json file.
```bash
{
  "canisters": {
    "icrc1_ledger_canister": {
      "type": "custom",
      "candid": "https://raw.githubusercontent.com/dfinity/ic/d87954601e4b22972899e9957e800406a0a6b929/rs/rosetta-api/icrc1/ledger/ledger.did",
      "wasm": "https://download.dfinity.systems/ic/d87954601e4b22972899e9957e800406a0a6b929/canisters/ic-icrc1-ledger.wasm.gz"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "output_env_file": ".env",
  "version": 1
}
```

If you chose to download the ICRC-1 ledger files with the script, you need to replace the Candid and Wasm file entries:

```bash
...
"candid": icrc1_ledger.did,
"wasm" : icrc1_ledger.wasm.gz,
  ...
```

In an existing project you would only need to add the icrc1_ledger_canister canister to the canisters section.


Step 5: Start a local replica.
```bash
dfx start --clean --background
```

```bash
dfx identity new minter
dfx identity use minter
export MINTER=$(dfx identity get-principal)

```
Transfers from the minting account will create Mint transactions. Transfers to the minting account will create Burn transactions.

Specify the token name and symbol of your choice:
```bash
export TOKEN_NAME="My Token"
export TOKEN_SYMBOL="XMTK"

```

Set the default identity or the identity with which you want to deploy the ledger.
```bash
dfx identity use default
export DEFAULT=$(dfx identity get-principal)
```


[OPTIONAL] To be able to interact and send some tokens you may want to mint some tokens when you deploy the ledger. You will mint some tokens for the default identity. You can also specify the transfer fee for transferring tokens.

```bash
export PRE_MINTED_TOKENS=10_000_000_000
export TRANSFER_FEE=10_000
```




The values set for archiving are the recommended values. You may change them to fit your ICRC-1 ledger's needs.


```bash
export FEATURE_FLAGS=false
```

If you want your ICRC-1 ledger to also support the extension standard ICRC-2 then set the flag to true:
```bash
export FEATURE_FLAGS=true
```

Step 7: Deploy the ICRC-1 ledger canister locally:

```bash
dfx deploy icrc1_ledger_canister --specified-id mxzaz-hqaaa-aaaar-qaada-cai --argument "(variant {Init =
record {
     token_symbol = \"${TOKEN_SYMBOL}\";
     token_name = \"${TOKEN_NAME}\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = ${TRANSFER_FEE};
     metadata = vec {};
     feature_flags = opt record{icrc2 = ${FEATURE_FLAGS}};
     initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; ${PRE_MINTED_TOKENS}; }; };
     archive_options = record {
         num_blocks_to_archive = ${NUM_OF_BLOCK_TO_ARCHIVE};
         trigger_threshold = ${TRIGGER_THRESHOLD};
         controller_id = principal \"${ARCHIVE_CONTROLLER}\";
         cycles_for_archive_creation = opt ${CYCLE_FOR_ARCHIVE_CREATION};
     };
 }
})"

```





## ICRC-1 endpoints
To fetch the symbol of the ICRC-1 ledger:
```bash
dfx canister call icrc1_ledger_canister icrc1_symbol '()'

```

To fetch the decimals of the ICRC-1 ledger:
```bash
dfx canister call icrc1_ledger_canister icrc1_decimals '()'

```

To fetch the total supply of the ICRC-1 ledger:
```bash
dfx canister call icrc1_ledger_canister icrc1_total_supply '()'

```

To fetch the fee of the ICRC-1 ledger:
```bash
dfx canister call icrc1_ledger_canister icrc1_fee '()'

```

To fetch the minting account of the ICRC-1 ledger:
```bash
dfx canister call icrc1_ledger_canister icrc1_minting_account '()'

```
To fetch the balance of an account (DEFAULT account in this case, with no subaccount set) on the ICRC-1 ledger:

```bash
dfx canister call icrc1_ledger_canister icrc1_balance_of "(record {owner = principal \"${DEFAULT}\"; })"

```
