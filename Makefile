build:
	cargo build

build-docker:
	docker build -t data_fetcher:dev .

run-local: build-docker
	docker run -p 9999:9999 -v ~/.config/data_fetcher/:/root/.config/data_fetcher data_fetcher:dev

run:
	docker compose up -d

clean:
	cargo clean && docker rmi -f data_fetcher:dev
