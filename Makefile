.PHONY: clippy-fixes clippy-strict

all: lint

lint:
	cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used -A clippy::module_name_repetitions -A clippy::needless_pass_by_value

run:
	cargo run --features "bevy/dynamic_linking, k8s, perfmon" -- --room-generator=k8s-live

static-run:
	cargo run --features "k8s, perfmon" -- --room-generator=k8s-live

test:
	cargo test --features k8s

debug-run:
	RUST_LOG=debug cargo run --features "bevy/dynamic_linking, k8s, perfmon" -- --room-generator=k8s-live

debug-test:
	RUST_LOG=debug cargo test --features k8s -- --nocapture
	#cargo test integration::k8s_live::k8s_api::tests::test_get_names_pods -- --nocapture
	#cargo test integration::k8s_live::k8s_api::tests::test_get_names_replicasets -- --nocapture

build:
	cargo build

clean:
	cargo clean
