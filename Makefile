ifneq (,$(wildcard ./.env))
include .env
export
endif

ifeq ($(SOFTWARE_NAME),misskey)
    REPO_URL := https://github.com/misskey-dev/misskey.git
    REPO_BRANCH := master
else ifeq ($(SOFTWARE_NAME),cherrypick)
    REPO_URL := https://github.com/kokonect-link/cherrypick.git
    REPO_BRANCH := master
else ifeq ($(SOFTWARE_NAME),sharkey)
    REPO_URL := https://activitypub.software/TransFem-org/Sharkey.git
    REPO_BRANCH := stable
endif

SRC := Cargo.toml

.PHONY: extract-version
extract-version:
    @mkdir -p temp && \
	echo "Cloning $(SOFTWARE_NAME) repository..." && \
    git clone $(REPO_URL) -b $(REPO_BRANCH) --recurse-submodules temp/$(SOFTWARE_NAME) && \
    @echo "Extracting version from $(SOFTWARE_NAME) ..." && \
	if [ -f temp/$(SOFTWARE_NAME)/package.json ]; then \
		VERSION=$$(jq -r '.version' temp/$(SOFTWARE_NAME)/package.json); \
		echo "Version: $$VERSION"; \
		export  SOFTWARE_VERSION=$$VERSION;\
	else \
		echo "Version not found in package.json"; \
		exit 1; \
	fi && \

.PHONY: generate-entities-mac
generate-entities-mac:
	@mkdir -p src/entities/$(SOFTWARE_NAME)/$(SOFTWARE_VERSION) && \
	sea-orm-cli generate entity \
		--output-dir='src/entities/$(SOFTWARE_NAME)/$(SOFTWARE_VERSION)' \
		--database-url='postgres://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@$(POSTGRES_HOST):$(POSTGRES_PORT)/$(POSTGRES_DB)' \
		--date-time-crate='chrono' \
		--with-serde='both' \
		--with-prelude='all-allow-unused-imports' && \
	gsed -i '3i use clap::ValueEnum;' src/entities/sea_orm_active_enums.rs && \
	gsed -i 's/#\[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)\]/#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ValueEnum)]/g' src/entities/sea_orm_active_enums.rs && \
	cargo fmt

.PHONY: migrate
migrate:
	$(MAKE) extract-version && \
	$(MAKE) generate-entities-mac
