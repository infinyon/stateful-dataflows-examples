BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)
include $(BASE_DIR)/Makefile

e2e:
	make -C $(CURDIR)/packages/mask-ssn build
	$(SDF_BIN) run --skip-running --dev
