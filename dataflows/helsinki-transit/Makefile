BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)
include $(BASE_DIR)/Makefile

e2e:
	$(SDF_BIN) run --skip-running

download-sm:
	fluvio hub smartmodule download infinyon/jolt@0.4.1

download-connector:
	cdk hub download infinyon/mqtt-source@0.2.9

start-connector:
	cdk deploy start --config connector/mqtt-helsinki.yaml --ipkg infinyon-mqtt-source-0.2.9.ipkg

shutdown-connector:
	cdk deploy shutdown --name helsinki-mqtt