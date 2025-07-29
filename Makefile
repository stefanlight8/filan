NAME := filan
TARGET ?=
PROFILE ?= release

ifeq ($(PROFILE),release)
	BUILD_PATH := target/$(TARGET)/release/$(NAME)
	CARGO_FLAGS := --release
else
	BUILD_PATH := target/$(TARGET)/debug/$(NAME)
	CARGO_FLAGS :=
endif

ifeq ($(findstring windows,$(TARGET)),windows)
	BUILD_PATH := $(BUILD_PATH).exe
endif

.PHONY: build run clean

build:
	@echo "==> Building $(NAME) $(if $(TARGET),for $(TARGET),for host) ($(PROFILE))"
	cargo build $(CARGO_FLAGS) $(if $(TARGET),--target $(TARGET))

run: build
	@echo "==> Running..."
	$(BUILD_PATH)

clean:
	cargo clean
