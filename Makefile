ENDPOINT ?= mainnet.eth.streamingfast.io:443
STOP_BLOCK ?= +10

# Deployement to `graph-node` config
IPFS_ENDPOINT ?= http://127.0.0.1:5001
GRAPH_NODE_ENDPOINT ?= http://127.0.0.1:8020
GRAPHMAN_CONFIG ?= ../graph-node-dev/config/graphman.toml

# Deployment to `substreams-sink-postgres` config
POSTGRESQL_DSN ?= psql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node?sslmode=disable

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run_db_out
run_db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -s 14204533 -t $(STOP_BLOCK)

.PHONY: run_graph_out
run_graph_out: build
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -s 14204533 -t $(STOP_BLOCK)

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package: build
	substreams pack ./substreams.yaml

.PHONE: sink_postgres_setup
sink_postgres_setup:
	substreams-sink-postgres setup --ignore-duplicate-table-errors "$(POSTGRESQL_DSN)" schema.sql

.PHONE: sink_postgres
sink_postgres:
	substreams-sink-postgres run -H "X-Sf-Substreams-Parallel-Jobs: 150" $(POSTGRESQL_DSN) $(ENDPOINT) "reth-erc20-rocket-v1.0.5.spkg" db_out 11446767:17576926

.PHONE: deploy_graph_node
deploy_graph_node: package
	mkdir build 2> /dev/null || true
	graph build --ipfs $(IPFS_ENDPOINT) subgraph.yaml
	graph create reth_benchmark --node $(GRAPH_NODE_ENDPOINT)
	graph deploy --node $(GRAPH_NODE_ENDPOINT) --ipfs $(IPFS_ENDPOINT) --version-label v0.0.1 reth_benchmark subgraph.yaml

.PHONE: undeploy_graph_node
undeploy_graph_node:
	graphman --config "$(GRAPHMAN_CONFIG)" drop --force block_meta