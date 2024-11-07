BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)
include $(BASE_DIR)/Makefile

generate:
	$(SDF_BIN) generate

build: generate
	$(SDF_BIN) build