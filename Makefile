VERSION = "0.0.1"
change-version:
	@echo $(VERSION)>VERSION
	
run-debug:
	PORT=8080 RUST_LOG=debug cargo run

run-release:
	PORT=8080 RUST_LOG=debug cargo run

build-debug:
	cargo build

build-release:
	cargo build --release

build-docker:
	docker build -t amjadjibon/echo-hyper .

run-docker-compose:
	docker-compose up