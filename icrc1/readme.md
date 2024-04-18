## ICRC1
ICRC1 is a token standard created by the Internet Computer working group. The acronym "ICRC" stands for "Internet Computer Request for Comments," and the ICRC1 standard is a standard for fungible tokens on the Internet Computer.

The ICRC1 standard defines a set of rules and functionalities for creating and recording token transactions on the Internet Computer. It specifies the general functionalities of ledgers, and any tokens and their corresponding ledgers that want to support the ICRC1 standard must fulfill all requirements within the standard.

The purpose of the ICRC1 standard is to create a universally accepted standard for fungible tokens on the Internet Computer, ensuring interoperability and compatibility across different token contracts and applications within the Internet Computer ecosystem.

By adhering to the ICRC1 standard, token contracts can ensure that they provide consistent and predictable behaviors for transferring tokens, querying token balances, and managing token allowances. This standardization allows fungible tokens to be seamlessly integrated into various decentralized applications (dApps) and smart contracts on the Internet Computer.

In summary, ICRC1 is a token standard that provides a common framework for creating fungible tokens on the Internet Computer, enabling developers to build and deploy token contracts that are compatible with a wide range of applications and services within the Internet Computer ecosystem.

## Data

### account

A `principal` can have multiple accounts. Each account of a `principal` is identified by a 32-byte string called `subaccount`. Therefore an account corresponds to a pair `(principal, subaccount)`.

The account identified by the subaccount with all bytes set to 0 is the _default account_ of the `principal`.

```candid "Type definitions" +=
type Subaccount = blob;
type Account = record { owner : principal; subaccount : opt Subaccount; };
```

## Methods

### icrc1_name <span id="name_method"></span>

Returns the name of the token (e.g., `MyToken`).

```candid "Methods" +=
icrc1_name : () -> (text) query;
```

### icrc1_symbol <span id="symbol_method"></span>

Returns the symbol of the token (e.g., `ICP`).

```candid "Methods" +=
icrc1_symbol : () -> (text) query;
```

### icrc1_decimals <span id="decimals_method"></span>

Returns the number of decimals the token uses (e.g., `8` means to divide the token amount by `100000000` to get its user representation).

```candid "Methods" +=
icrc1_decimals : () -> (nat8) query;
```

### icrc1_fee <span id="fee_method"></span>

Returns the default transfer fee.

```candid "Methods" +=
icrc1_fee : () -> (nat) query;
```

### icrc1_metadata <span id="metadata_method"></span>

Returns the list of metadata entries for this ledger.
See the "Metadata" section below.

```candid "Type definitions" +=
type Value = variant { Nat : nat; Int : int; Text : text; Blob : blob };
```

```candid "Methods" +=
icrc1_metadata : () -> (vec record { text; Value }) query;
```

### icrc1_total_supply

Returns the total number of tokens on all accounts except for the [minting account](#minting_account).

```candid "Methods" +=
icrc1_total_supply : () -> (nat) query;
```

### icrc1_minting_account

Returns the [minting account](#minting_account) if this ledger supports minting and burning tokens.

```candid "Methods" +=
icrc1_minting_account : () -> (opt Account) query;
```

### icrc1_balance_of

Returns the balance of the account given as an argument.

```candid "Methods" +=
icrc1_balance_of : (Account) -> (nat) query;
```

### icrc1_transfer <span id="transfer_method"></span>

Transfers `amount` of tokens from account `record { of = caller; subaccount = from_subaccount }` to the `to` account.
The caller pays `fee` tokens for the transfer.

```candid "Type definitions" +=
type TransferArgs = record {
    from_subaccount : opt Subaccount;
    to : Account;
    amount : nat;
    fee : opt nat;
    memo : opt blob;
    created_at_time : opt nat64;
};

type TransferError = variant {
    BadFee : record { expected_fee : nat };
    BadBurn : record { min_burn_amount : nat };
    InsufficientFunds : record { balance : nat };
    TooOld;
    CreatedInFuture : record { ledger_time: nat64 };
    Duplicate : record { duplicate_of : nat };
    TemporarilyUnavailable;
    GenericError : record { error_code : nat; message : text };
};
```

```candid "Methods" +=
icrc1_transfer : (TransferArgs) -> (variant { Ok: nat; Err: TransferError; });
```

The caller pays the `fee`.
If the caller does not set the `fee` argument, the ledger applies the default transfer fee.
If the `fee` argument does not agree with the ledger fee, the ledger MUST return `variant { BadFee = record { expected_fee = ... } }` error.

The `memo` parameter is an arbitrary blob that has no meaning to the ledger.
The ledger SHOULD allow memos of at least 32 bytes in length.
The ledger SHOULD use the `memo` argument for [transaction deduplication](#transaction_deduplication).

The `created_at_time` parameter indicates the time (as nanoseconds since the UNIX epoch in the UTC timezone) at which the client constructed the transaction.
The ledger SHOULD reject transactions that have `created_at_time` argument too far in the past or the future, returning `variant { TooOld }` and `variant { CreatedInFuture = record { ledger_time = ... } }` errors correspondingly.

The result is either the transaction index of the transfer or an error.

### icrc1_supported_standards

Returns the list of standards this ledger implements.
See the ["Extensions"](#extensions) section below.

```candid "Methods" +=
icrc1_supported_standards : () -> (vec record { name : text; url : text }) query;
```

The result of the call should always have at least one entry,

```candid
record { name = "ICRC-1"; url = "https://github.com/dfinity/ICRC-1" }
```

## Extensions <span id="extensions"></span>

The base standard intentionally excludes some ledger functions essential for building a rich DeFi ecosystem, for example:

  - Reliable transaction notifications for smart contracts.
  - The block structure and the interface for fetching blocks.
  - Pre-signed transactions.

The standard defines the `icrc1_supported_standards` endpoint to accommodate these and other future extensions.
This endpoint returns names of all specifications (e.g., `"ICRC-42"` or `"DIP-20"`) implemented by the ledger.

## Metadata

A ledger can expose metadata to simplify integration with wallets and improve user experience.
The client can use the [`icrc1_metadata`](#metadata_method) method to fetch the metadata entries. 
All the metadata entries are optional.

### Key format

The metadata keys are arbitrary Unicode strings and must follow the pattern `<namespace>:<key>`, where `<namespace>` is a string not containing colons.
Namespace `icrc1` is reserved for keys defined in this standard.

### Standard metadata entries
| Key | Semantics | Example value
| --- | ------------- | --------- |
| `icrc1:symbol` | The token currency code (see [ISO-4217](https://en.wikipedia.org/wiki/ISO_4217)). When present, should be the same as the result of the [`icrc1_symbol`](#symbol_method) query call. | `variant { Text = "XTKN" }` | 
| `icrc1:name` | The name of the token. When present, should be the same as the result of the [`icrc1_name`](#name_method) query call. | `variant { Text = "Test Token" }` | 
| `icrc1:decimals` |  The number of decimals the token uses. For example, 8 means to divide the token amount by 10<sup>8</sup> to get its user representation. When present, should be the same as the result of the [`icrc1_decimals`](#decimals_method) query call. | `variant { Nat = 8 }` |
| `icrc1:fee` | The default transfer fee. When present, should be the same as the result of the [`icrc1_fee`](#fee_method) query call. |  `variant { Nat = 10_000 }` |
| `icrc1:logo` | The URL of the token logo. The value can contain the actual image if it's a [Data URL](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs).  | `variant { Text = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMSIgaGVpZ2h0PSIxIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxyZWN0IHdpZHRoPSIxMDAlIiBoZWlnaHQ9IjEwMCUiIGZpbGw9InJlZCIvPjwvc3ZnPg==" }` | 





## ICRC-1 ledger setup
* Step 1: Make sure you use a recent version of the IC SDK.

### download icrc-1 ledger wasm

- download latest icrc-1 [ledger](https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/deploy-new-token) wasm and did
- to download them, we need the commit hash of the last [blessed replica](https://dashboard.internetcomputer.org/releases) version
    - basically replica version that was approved by the nns to be running on nodes
- with the commit hash we run the following commands to download the files

```bash
curl -o icrc1.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz"
curl -o icrc1.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/ledger.did"
gunzip icrc1.wasm.gz
```

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

* Step 4: Configure the dfx.json file.
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



* Step 6: Create the required identities and export initialization arguments
Transfers from the minting account will create Mint transactions. Transfers to the minting account will create Burn transactions.


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

The initialization arguments of the ICRC-1 ledger are not specified in the standard. Thus, the arguments defined in this section are dependent on the reference implementation of the ICRC-1 ledger. If you build your own ICRC-1 ledger you may use different initialization arguments.


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
dfx identity new archive_controller
dfx identity use archive_controller
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)
export TRIGGER_THRESHOLD=2000
export NUM_OF_BLOCK_TO_ARCHIVE=1000
export CYCLE_FOR_ARCHIVE_CREATION=10000000000000

```
Specify which standards to support. If you only want to support the ICRC-1 standard then you can set:

```bash
export FEATURE_FLAGS=false
```

If you want your ICRC-1 ledger to also support the extension standard ICRC-2 then set the flag to true:
```bash
export FEATURE_FLAGS=true
```

Check the set variables:

For each variable, the exported environment variable will be used unless otherwise specified:

* The PRE_MINTED_TOKENS is amount of tokens that are minted during deployment for a specific account (In this tutorial it will be the DEFAULT account).
* The TRANSFER_FEE is the transfer fee that users of the ledger will have to pay anytime they want to make a transfer.
* The ARCHIVE_CONTROLLER is the controller principal of the archive canisters.
* The TRIGGER_THRESHOLD is the number of blocks to archive when the trigger threshold is exceeded.
* The CYCLE_FOR_ARCHIVE_CREATION is the amount of cycles that will be sent to the archive canister when it is created.
* The NUM_OF_BLOCK_TO_ARCHIVE is the number of blocks that will be archived.
* The TOKEN_SYMBOL is the ticker symbol of your new token.
* The MINTER is the account of the Principal responsible for minting and burning tokens (see the icrc-1 ledger documentation).
* The FEATURE_FLAGS is a flag for enabling or disabling certain extension standards to the ICRC-1 standard. In this case the reference implementation also can support ICRC-2 transactions.
* Minting 100 tokens to the DEFAULT (1 token is by default equal to 10^8 e8s, hence the name).
* Setting the transfer fee to 0.0001 tokens.

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



## Step 8 Deploy the ledger canister

```bash
dfx deploy --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai icp_ledger_canister --argument "
  (variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record {
            e8s = 10_000_000_000 : nat64;
          };
        };
      };
      send_whitelist = vec {};
      transfer_fee = opt record {
        e8s = 10_000 : nat64;
      };
      token_symbol = opt \"LICP\";
      token_name = opt \"Local ICP\";
    }
  })
"
```
Take a moment to read the details of the call made above. Not only are you deploying the ICP ledger canister, you are also:

* Deploying the canister to the same canister ID as the mainnet ledger canister. This is to make it easier to switch between local and mainnet deployments.
* Setting the minting account to the account identifier you saved in a previous step (MINTER_ACCOUNT_ID).
* Minting 100 ICP tokens to the DEFAULT_ACCOUNT_ID (1 ICP is equal to 10^8 e8s, hence the name).
* Setting the transfer fee to 0.0001 ICP.
* Naming the token Local ICP / LICP

## Step 9: Interact with the canister.
You can interact with the canister by running CLI commands, such as:


```bash
dfx canister call icp_ledger_canister name
```

This command will return the token's name, such as:


```bash
("Local ICP")

```
Or, you can interact with it using the Candid UI by navigating to the URL provided when the canister was deployed, such as:

```bash
http://127.0.0.1:4943/?canisterId=bnz7o-iuaaa-aaaaa-qaaaa-cai&id=ryjl3-tyaaa-aaaaa-aaaba-cai

```


- Your local ICP ledger canister is up and running. You can now deploy other canisters that need to communicate with the ledger canister.


Step 10: Testing your ICRC-1 implementation

```bash
# ==========
# With Cargo
# ==========

$ cargo run --bin icrc1-test-runner -- -u REPLICA_URL -c CANISTER_ID -s identity.pem

# ==========
# With Bazel
# ==========

$ bazel run //test/runner -- -u REPLICA_URL -c CANISTER_ID -s identity.pem

# for example

$ bazel run //test/runner -- -u http://localhost:9000 -c rrkah-fqaaa-aaaaa-aaaaq-cai -s ~/.config/dfx/identity/test/identity.pem

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
