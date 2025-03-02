ifneq (,$(wildcard ./.env))
include .env
export
endif

SRC := Cargo.toml

.PHONY: generate-entities-mac
generate-entities:
	rm -rf src/entities 
	sea-orm-cli generate entity \
		--output-dir='src/entities' \
		--database-url='postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@$(POSTGRES_HOST):$(POSTGRES_PORT)/$(POSTGRES_DB)' \
		--date-time-crate='chrono' \
		--with-serde='both' \
		--with-prelude='all-allow-unused-imports' && \
	gsed -i '3i use clap::ValueEnum;' src/entities/sea_orm_active_enums.rs && \
	gsed -i 's/#\[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)\]/#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ValueEnum)]/g' src/entities/sea_orm_active_enums.rs && \
	cargo fmt
