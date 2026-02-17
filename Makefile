.PHONY: help build-docker

help: Makefile ## Print help
	@grep -h "##" $(MAKEFILE_LIST) | grep -v grep | sed -e 's/:.*##/#/' | column -c 2 -t -s#

build-docker: ## Build local Docker
	docker build -t thedate .

build: ## Cargo build
	cargo build

build-release: ## Cargo build --release
	cargo build --release

run: ## Cargo run
	cargo run

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with output
	cargo test -- --nocapture

lint: ## Run clippy linter
	cargo clippy -- -D warnings

fmt: ## Format code with rustfmt
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt --check

check: fmt-check lint test ## Run all quality checks (fmt, lint, test)

clean: ## Clean build artifacts
	cargo clean

docker-build: build-docker ## Alias for build-docker

docker-run: ## Run Docker container locally
	docker run -p 8080:8080 --name thedate-dev thedate

docker-stop: ## Stop and remove running container
	docker stop thedate-dev && docker rm thedate-dev

docker-test: ## Build and test in Docker
	docker build -t thedate-test . && \
	docker run --rm thedate-test cargo test

compose-up: ## Start services with docker-compose
	docker-compose up -d

compose-down: ## Stop services
	docker-compose down

compose-logs: ## View logs
	docker-compose logs -f thedate
