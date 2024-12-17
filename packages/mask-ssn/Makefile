BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)
include $(BASE_DIR)/Makefile

update:
	$(SDF_BIN) update || true # do not fail if update fails (update for beta4)
	echo '\nregex = "1.1"\n' >> rust/mask-ssn/Cargo.toml

build: update
	$(SDF_BIN) build
