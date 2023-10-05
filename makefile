PROJECT_NAME = climp

TARGET_DIR = target/release

INSTALL_DIR = /usr/local/bin

TEST_DIR = tests

EXECUTABLE = $(PROJECT_NAME)

RUSTC = rustc

all: build

build:
	@echo "Building..."
	@cargo build --release
	@echo "Built"

install: build
	@echo "Installing..."
	@install -m 755 $(TARGET_DIR)/$(EXECUTABLE) $(INSTALL_DIR)
	@echo "Installed"
	@clean

test: clean-tests build 
	@echo "Running..."
	@install -m 777 $(TARGET_DIR)/$(EXECUTABLE) $(TEST_DIR)
	cd $(TEST_DIR) && python3.11 tests.py

clean:
	@echo "Cleaning up..."
	@cargo clean
	@echo "Cleaned up"

clean-tests:
	@echo "Cleaning up..."
	@find $(TEST_DIR) -mindepth 1 -maxdepth 1 ! -name 'tests.py' -exec rm -rf {} +

.PHONY: all build install clean