#!make
project-name = rusty

up:
	@echo Start $(project-name) API Service: && \
	docker compose up -d
down:
	@echo Shut down $(project-name) API Service: && \
	docker compose down
docker-build:
	@echo Build $(project-name) API Service docker image: && \
	docker build --no-cache -t rusty .
local-build:
	@echo Build $(project-name) API Service local image: && \
	cargo clean && cargo build
reset:
	@echo [WARNING] Reset $(project-name) project: && \
	docker compose down --rmi all --remove-orphans
	docker volume prune
	docker image prune
	sudo rm -rf mysql-db-primary
setup-db:
	@echo Setup $(project-name) DB: && \
	./platform/bin/db-setup.sh
