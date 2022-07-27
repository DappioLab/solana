# `solana-mf`: Solana Mainnet Fork for Better Developer Experience

- **Check the PoC [here](https://explorer.solana.com/?cluster=custom&customUrl=https%3A%2F%2Frpc-mainnet-fork.dappio.xyz)**

## TL; DR

`solana-mf` restores the entire mainnet account data locally. Developers now can build and test with mainnet account data on their tesing environmant. This is especially handy for testing program that needs to interact with different programs.

## Motivation

Imagine you are trying to build your DeFi lego on top of a very complex program. To test your DApp with the program, one of the following cases will heppen:

### Case 1

**The team has set up the devnet / testnet program for you.** Great! But this is not the usual case since it needs extra effort from the team for maintenance.

### Case 2

**The team doesn't set up the devnet. However, the program is open-sourced.** You have to set up the devnet yourself. This is acceptable but it gets you frustrated a lot. Still, you need to dedicate decent amount of time doing so.

### Case 3

**The team doesn't setup the devnet and the program is not open-sourced.** Things begin to get nasty here. To test your DApp, clone mainnet data to your local test validator seems to be the only option. You can use `--clone` to achieve so. **However, there are a few downsides of this approach:**

- **Hard to maintain**: Need to specify a long list if you try to interact with a complex program
- **Unpredicted scope**: Need to get every possible interactive account, which sometimes is not possible
- **Prone to error**: Downloading takes a long time and easy to break if the account to be fetched does not exist
- **Limited size**: It has size limitation (10 MB)

### Our Approach

Instead of downloading account one by one, we choose to restore the entire account set from mainnet-beta snapshot, in other words, **mainnet-fork**.

With `solana-mf`, developers don't have to deal with error-proning testing environment anymore. This is extremely helpful when you try to integrate programs that are not deployed on tesetnet / devnet.

## Requirement

- 150+GB RAM + Swap (For account index)
- It takes **~3 hours** to restore snapshot with 4 Core CPU + 32GB RAM + 200GB Swap

## Setup

### Structure

```
├── 📂 solana-mf
│
├── 📂 cacherpc
│
└── 📂 my-test-validator
    │
    ├── 📂 snapshots
    │   │
    │   └── 📄 snapshot-107470451-....tar.zst
    │
    ├── 📂 test-ledger
    │
    ├── 📄 solana-mf.log
    │
    └── 📄 cacherpc.log
```

### Install `solana-mf`

Clone the repo to folder `solana-mf`:

```bash
$ git clone git@github.com:DappioWonderland/solana.git -b solana-mf --single-branch solana-mf
```

Build `solana-mf` from source:

```
$ cd solana-mf
$ ./scripts/cargo-install-all.sh .
```

Export the built `bin` folder to `$PATH`:

```bash
$ export PATH=$PWD/bin:$PATH
```

### Install `cacherpc`

Here we use [`cacherpc`](https://github.com/zubr-exchange/cacherpc) to handle the RPC requests so that we can boost the poor performance of certain RPC calls such as [`getProgramAccounts`](https://docs.solana.com/developing/clients/jsonrpc-api#getprogramaccounts).

Let's clone `cache-rpc`:

```
$ git clone git@github.com:zubr-exchange/cacherpc.git
```

Config `REQUEST_TIMEOUT` from `30` to `3600`:

```rust
// In src/rpc.rs
// Line 39

const REQUEST_TIMEOUT: Duration = Duration::from_secs(3600);
```

Build `cacherpc` from source:

```bash
$ cd cacherpc
$ cargo build --release
```

### Download Snapshot

Create a new folder to place ledger and snapshot:

```
$ mkdir -p my-test-validator/snapshots
```

Download mainnet-beta snapshot:

```
$ wget --trust-server-names http://api.mainnet-beta.solana.com/snapshot.tar.bz2
```

Move to snapshot file to `my-test-validator/snapshots` when the download is completed:

```bash
$ mv snapshot-107470451-7pz7qNevK2Phy35yps6ETRiZzJEbzzjnJtehat9yn6Am.tar.zst ./my-test-validator/snapshots
```

### Start

Now, let's run `solana-test-validator`:

```bash
$ cd my-test-validator
$ nohup solana-test-validator --account-index program-id --account-index spl-token-owner --account-index spl-token-mint --gossip-host YOUR_DOMAIN_OR_IP_ADDRESS -w 107470452 > solana-mf.log 2>&1 &
```

Notice that:

1. **The parameter of `-w` flag has to be larger than the slot of snapshot by `1`**. In our example, the slot of downloaded snapshot is `107470451` thus we have `107470452` as the parameter.
2. **The paramter of `--gossip-host` flag is the domain or IP address of your test validator.** Ex: if your IP address is `http://1.2.3.4`, use `1.2.3.4` as the parameter. The default vaule is `127.0.0.1` so it's ok to remove this flag if you attempt to use `solana-mf` locally.

The validator is going take from minutes to **hours** to start, depending on your hardware. In our case, it takes ~3 hours to restore snapshot with 4 Core CPU + 32GB RAM + 200GB Swap

Once the `solana-test-validator` is up, you will see this log in `solana-mf.log`:

```
Ledger location: test-ledger
Log: test-ledger/validator.log
Initializing...
Waiting for fees to stabilize 1...
Connecting...
Identity: 7rn71LeohqddXZAokoibaNxoeryrydBSdasCjDY9zVwv
Genesis Hash: 3k99X14twmQDGS4CmAswM1WQqUN3NNyxAP6LFMzS9Hh4
Version: 1.9.0
Shred Version: 9276
Gossip Address: 127.0.0.1:1024
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
02:58:23 | Processed Slot: 107470454 | Confirmed Slot: 107470454 | Finalized Slot: 107470452 | Full Snapshot Slot: 107470451 | Incremental Snapshot Slot: - | Transactions: 40019709625 | ◎1000.000000000
02:58:23 | Processed Slot: 107470455 | Confirmed Slot: 107470455 | Finalized Slot: 107470452 | Full Snapshot Slot: 107470451 | Incremental Snapshot Slot: - | Transactions: 40019709625 | ◎1000.000000000
02:58:23 | Processed Slot: 107470455 | Confirmed Slot: 107470455 | Finalized Slot: 107470452 | Full Snapshot Slot: 107470451 | Incremental Snapshot Slot: - | Transactions: 40019709625 | ◎1000.000000000
02:58:24 | Processed Slot: 107470456 | Confirmed Slot: 107470456 | Finalized Slot: 107470452 | Full Snapshot Slot: 107470451 | Incremental Snapshot Slot: - | Transactions: 40019709625 | ◎1000.000000000
02:58:24 | Processed Slot: 107470456 | Confirmed Slot: 107470456 | Finalized Slot: 107470452 | Full Snapshot Slot: 107470451 | Incremental Snapshot Slot: - | Transactions: 40019709627 | ◎999.999990000
02:58:24 | Processed Slot: 107470457 | Confirmed Slot: 107470457 | Finalized Slot: 107470452 | Full Snapshot Slot: 107470451 | Incremental Snapshot Slot: - | Transactions: 40019709627 | ◎999.999990000
...
```

> Chances are you have to use `sys-tuner` or add more swap.

Finally, let's run `cacherpc`:

```bash
$ nohup ../cacherpc/target/release/cache-rpc -r http://localhost:8899 -w ws://localhost:8900 -l 0.0.0.0:9001 -t 604800sec > cacherpc.log 2>&1 &
```

Notice that the time-to-live of the account cache is set via the flag `-t`. We use `604800sec` (~1 week) as the parameter of `-t` flag since the freshness of the data is not the first concern in our case. This can be changed due to your test case.

### Test

Let's send a [getAccountInfo](https://docs.solana.com/developing/clients/jsonrpc-api#getaccountinfo) RPC call of Serum v3 Program `9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin` to test if `solana-mf` is functioning successfully:

```
$ curl http://YOUR_DOMAIN_OR_IP_ADDRESS:9001 -X POST -H "Content-Type: application/json" -d '
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getAccountInfo",
    "params": [
      "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
      {
        "encoding": "base58"
      }
    ]
  }
'

{"jsonrpc":"2.0","result":{"context":{"slot":107629792},"value":{"data":["t64jZRB3bKxxYQTtQ976jXJxgFRKEh7vhNf42Vuq4MKWeEND","base58"],"executable":true,"lamports":1141440,"owner":"BPFLoaderUpgradeab1e11111111111111111111111","rentEpoch":148}},"id":1}
```

Join our [discord dev channel](https://discord.com/invite/ZsVcwV6D57) to discuss more.
