target = thumbv6m-none-eabi

cargo-native = cargo
cargo-target = xargo

native-args = --features "test"
test-args = --features "test serial svc minicom"
target-args = --target=$(target)

.PHONY: native native-release target clean

all: native

native:
	@$(cargo-native) build $(native-args)

native-release:
	@$(cargo-native) build $(native-args) --release

test:
	@$(cargo-native) test $(test-args)

target:
	@$(cargo-target) build $(target-args)

clean:
	@$(cargo-native) clean
