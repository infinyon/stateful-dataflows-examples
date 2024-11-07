BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)
include $(BASE_DIR)/Makefile

e2e:
	$(SDF_BIN) run --skip-running