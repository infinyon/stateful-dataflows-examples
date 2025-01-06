BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)
include $(BASE_DIR)/Makefile

e2e:
	$(SDF_BIN) run --skip-running


download-locations:
	curl -s https://demo-data.infinyon.com/api/ny-transit/locations | fluvio produce locations
download-vendors:
	curl -s https://demo-data.infinyon.com/api/ny-transit/vendors  | fluvio produce vendors