dev:
	cargo watch -x 'run'

clean:
	rm -rf ./bin

build:
	cargo build --release --target-dir ./bin

run:
	./bin/release/talino-tap-api

loadtest:
	k6 run ./script.js
