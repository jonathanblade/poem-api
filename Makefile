pg-up:
	docker-compose -f docker-compose.dev.yml up -d

pg-down:
	docker-compose -f docker-compose.dev.yml down -v

build:
	docker build -t poem-example-app .

clean:
	rm -rf poem-example-app.db*

test:
	cargo test -- --test-threads 1
