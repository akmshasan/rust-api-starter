up:
	@docker compose up -d --build

down:
	@docker compose down -v

run:
	@docker compose exec app cargo run

test:
	@docker compose exec app cargo test

.PHONY: up down run test