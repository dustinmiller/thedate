.PHONY: help build-docker

help: Makefile ## Print help
	@grep -h "##" $(MAKEFILE_LIST) | grep -v grep | sed -e 's/:.*##/#/' | column -c 2 -t -s#

build-docker: ## Build local Docker
	docker build -t thedate .

build: ## Cargo build
	cargo build

run: ## Cargo run
	cargo run
