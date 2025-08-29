ifneq (,$(wildcard ./.env))
include .env
export
endif

ifneq (,$(wildcard ./.env.software))
include .env.software
export
endif

SRC := Cargo.toml

.PHONY: up
up:
	docker compose --env-file .env.software -f compose.local.yml up -d

.PHONY: down
down:
	docker compose --env-file .env.software -f compose.local.yml down

.PHONY: reset-db
reset-db:
	@echo "Resetting database..."
	docker compose --env-file .env.software -f compose.local.yml down -v
	docker compose --env-file .env.software -f compose.local.yml rm -f db
	docker volume prune -f
	sudo rm -rf ./db/*
	sudo rm -rf ./redis/*
	sudo rm -rf ./meilisearch/*
	@echo "Database reset complete."

.PHONY: db-init
db-init:
	sudo docker compose --env-file .env.software -f compose.local.yml run --rm web pnpm run init

.PHONY: generate-entities-mac
generate-entities-mac:
	$(MAKE) reset-db && \
	$(MAKE) db-init && \
	$(MAKE) up && \
	sleep 10 && \
	mkdir -p src/entities/$(SOFTWARE_NAME)/$(SOFTWARE_VERSION) && \
	sea-orm-cli generate entity \
		--output-dir='src/entities/$(SOFTWARE_NAME)/$(SOFTWARE_VERSION)' \
		--database-url='postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@$(POSTGRES_HOST):$(POSTGRES_PORT)/$(POSTGRES_DB)' \
		--date-time-crate='chrono' \
		--with-serde='both' \
		--with-prelude='all-allow-unused-imports' && \
	cargo fmt && \
	$(MAKE) down

.PHONY: clean-cache
clean-cache:
	cargo clean
