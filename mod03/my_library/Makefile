SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


clean: ## remove all generated files
	cargo clean

build: ## Build the project
	cargo build

lint: ## Lint the code
	@rustup component add clippy 2> /dev/null
	cargo clippy

format: ## Format the code
	@rustup component add rustfmt 2> /dev/null
	cargo fmt