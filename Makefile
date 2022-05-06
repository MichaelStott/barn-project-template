sdl2: ## Download SDL2 dependencies
	@echo "TODO"

build: ## Pull dependencies and build project 
	CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build

run: ## Run project in dev mode
	cargo run

release: ## Build EXE folder and distributable project folder
	cargo build --release