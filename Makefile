.DEFAULT_GOAL := help

# Cargo features for builds.
FEATURES ?=

# Cargo profile for builds.
PROFILE ?= release

# Extra flags for Cargo.
CARGO_INSTALL_EXTRA_FLAGS ?=

##@ Help
.PHONY: help
help: # Display this help.
	@awk 'BEGIN {FS = ":.*#"; printf "Usage:\n  make \033[34m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?#/ { printf "  \033[34m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) }' $(MAKEFILE_LIST)

##@ Build
.PHONY: build
build: # Build the Ream binary into `target` directory.
	cargo build --bin ream --features "$(FEATURES)" --profile "$(PROFILE)"

##@ Fmt
.PHONY: fmt
fmt:
	cargo fmt
	(cd methods/guest && cargo fmt)

##@ Dev
.PHONY: dev
dev: # Run in dev mode
	RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 RUST_BACKTRACE=0 cargo run --release

##@ Prove
.PHONY: prove
prod: # Run with real proof generation
	RUST_LOG="[executor]=info" RISC0_DEV_MODE=0 RUST_BACKTRACE=0 cargo run --release
