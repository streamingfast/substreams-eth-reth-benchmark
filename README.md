## Substreams Ethereum Reth Benchmark

This repository contains an equivalent Substreams as the one presented in the Reth benchmark but this version is using Substreams best practices.

### Instructions

1. Download and install https://github.com/streamingfast/substreams-sink-postgres/releases/tag/v2.3.4.

   > **Note** Latest release could be used also

1. Install Substreams CLI and dependencies https://substreams.streamingfast.io/getting-started/installing-the-cli.

1. Get you an API key and generate an API token from it https://substreams.streamingfast.io/getting-started/quickstart#run-your-first-substreams.

1. Using Docker, run `./up.sh -c` which essentially runs `docker-compose up`.

1. Open another terminal, go to this project again

1. Run `substreams-sink-postgres setup "psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable" schema.sql` to bootstrap the database with the correct tables.

1. Run `substreams-sink-postgres run "psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable" mainnet.eth.streamingfast.io:443 "reth-erc20-rocket-v1.0.6.spkg" db_out 11446767:17576926`.

#### Fresh Cache

By using one of the pre-compiled `.spkg`, you are leveraging an existing cache on the Substreams backend which essentially stream you out the data back. To replicate a fast ingestion speed on fresh cache, you need to have access to high amount of  max Substreams parallel jobs, contact us through [Discord](https://discord.com/channels/666749063386890256/820011680842907761) to experiment with one.

Update the `Cargo.toml` `version` field by one and the `substreams.yaml` `version` (**not** `specVersion`!) field so they are aligned. Generate a new `.spkg` with `substreams package`. This should yield a file `reth-erc20-rocket-v1.0.x.spkg`.

Check that `substreams info reth-erc20-rocket-v1.0.x.spkg` gives a different hash for `db_out` than the hash for `db_out` but in `reth-erc20-rocket-v1.0.6.spkg`.

Then re-run the command above with the high parallel jobs count set to 150:

```
substreams-sink-postgres run -H "X-Sf-Substreams-Parallel-Jobs: 150" "psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable" mainnet.eth.streamingfast.io:443 "reth-erc20-rocket-v1.0.x.spkg" db_out 11446767:17576926
```

> **Note** Don't forget to pick the right `reth-erc20-rocket-v1.0.x.spkg` file!

To validate that the your are correctly configured to use 150 max parallel jobs, look for the log line `session initialized with remote endpoint` which list the how many maximum parallel jobs the server will allocate. It should look like this:

```
2023-07-13T08:11:15.003-0400 INFO (sink-postgres) session initialized with remote endpoint {"max_parallel_workers": 150, "linear_handoff_block": 17684300, "resolved_start_block": 0, "trace_id": "7e90307e13152e6d3f28f72e514e6bcb"}
```

If you see `"max_parallel_workers": 15` or less, your API token is not configured to use more than the default number of max parallel jobs the server allows. This is most likely the case if you simply generated an API token without talking to us first. Once we told you it was activated, ensure you have generated a new API **token** (and **not** key). Contact us through [Discord](https://discord.com/channels/666749063386890256/820011680842907761) if you want to run the benchmark and need the special configuration allowing 150 max parallel jobs.
