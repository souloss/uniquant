# ==============================================================================
# SeaORM Project Makefile
# ==============================================================================
# This Makefile provides a standardized set of commands for managing
# SeaORM migrations and entity generation.
#
# Usage:
#   make help           # Show this help message
#   make migrate-add    # Create a new migration file
#   make migrate-up     # Apply all pending migrations
#   make migrate-down   # Rollback the last migration
#   make migrate-status # Show migration status
#   make generate-entity # Generate entities from the database
#   make clean          # Clean generated files
# ==============================================================================

# --- Configuration ---
# Set your database URL here.
# SQLite Example:
DATABASE_URL = sqlite:$(CURDIR)/uniquant.db?mode=rwc
# PostgreSQL Example:
# DATABASE_URL = postgres://user:password@localhost/database

# Output directory for generated entities
ENTITY_OUTPUT_DIR = src/entity
# Output directory for generated dto
DTO_OUTPUT_DIR = src/dto/generated
# Optional: Generate serde derives for entities
# Options: "both", "serialize", "deserialize", or "" (empty to disable)
WITH_SERDE = both

# Migration directory (usually 'migration')
MIGRATION_DIR = src/migration

# --- Commands ---
# The command to run sea-orm-cli for migrations.
# It runs the migration binary from the migration directory.
MIGRATION_CMD = cd $(MIGRATION_DIR) && cargo run --manifest-path ./Cargo.toml --

# The command to run sea-orm-cli for entity generation.
# Assumes sea-orm-cli is installed globally or in your PATH.
# If not, you can use: `cargo run --bin sea-orm-cli --`
GENERATE_CMD = sea-orm-cli

# --- Phony Targets ---
# Declare all targets as phony to avoid conflicts with files of the same name.
.PHONY: help init migrate-add migrate-up migrate-down migrate-status generate-entity clean

# --- Default Target ---
# The 'help' target will run if you just type 'make'.
help:
	@echo "SeaORM Management Commands:"
	@echo "---------------------------"
	@echo "  help               Show this help message."
	@echo "  init               Initialize a new migration project (one-time setup)."
	@echo ""
	@echo "  migrate-add        Create a new migration file."
	@echo "    Usage: make migrate-add NAME=create_user_table"
	@echo ""
	@echo "  migrate-up         Apply all pending migrations to the database."
	@echo "  migrate-down       Rollback the last applied migration."
	@echo "  migrate-status     Show the current status of migrations."
	@echo ""
	@echo "  generate-entity    Generate entities from the database schema."
	@echo "  generate-dto       Generate dto from the database schema."
	@echo "                     (This will automatically run 'migrate-up' first)."
	@echo ""
	@echo "  clean              Remove the database file and generated entities."
	@echo ""
	@echo "Configuration:"
	@echo "  DTO_OUTPUT_DIR：   $(DTO_OUTPUT_DIR)"
	@echo "  DATABASE_URL:      $(DATABASE_URL)"
	@echo "  ENTITY_OUTPUT_DIR: $(ENTITY_OUTPUT_DIR)"
	@echo "  WITH_SERDE:        $(WITH_SERDE)"

# --- Migration Targets ---

init:
	@echo "Initializing migration project in '$(MIGRATION_DIR)'..."
	$(MIGRATION_CMD) init

migrate-add:
	@if [ -z "$(MIG_NAME)" ]; then \
		echo "Error: Migration name is required."; \
		echo "Usage: make migrate-add MIG_NAME=<migration_name>"; \
		exit 1; \
	fi
	@echo "Creating new migration: $(MIG_NAME)"
	$(MIGRATION_CMD) generate $(MIG_NAME)

migrate-up:
	@echo "Running database migrations..."
	$(MIGRATION_CMD) up

migrate-down:
	@echo "Rolling back the last migration..."
	$(MIGRATION_CMD) down

migrate-status:
	@echo "Checking migration status..."
	$(MIGRATION_CMD) status

# --- Entity Generation Target ---

generate-entity: migrate-up
	@echo "Generating entities from database..."
	@if [ -n "$(WITH_SERDE)" ]; then \
		SERDE_FLAG="--with-serde $(WITH_SERDE)"; \
	else \
		SERDE_FLAG=""; \
	fi; \
	$(GENERATE_CMD) generate entity \
		-u "$(DATABASE_URL)" \
		-o "$(ENTITY_OUTPUT_DIR)" \
		$$SERDE_FLAG
	@echo "Entity generation complete."

generate-dto: migrate-up
	@echo "Generating dto from database..."
	@if [ -n "$(WITH_SERDE)" ]; then \
		SERDE_FLAG="--with-serde $(WITH_SERDE)"; \
	else \
		SERDE_FLAG=""; \
	fi; \
	$(GENERATE_CMD) generate entity \
		--frontend-format \
		-u "$(DATABASE_URL)" \
		-o "$(DTO_OUTPUT_DIR)" \
		$$SERDE_FLAG
	@echo "DTO generation complete."
# --- Utility Target ---

clean:
	@echo "Cleaning up..."
	@echo "Removing database file..."
	@rm -f ./uniquant.db
	@echo "Removing generated entities..."
	@rm -rf $(ENTITY_OUTPUT_DIR)/*.rs
	@echo "Clean complete."
