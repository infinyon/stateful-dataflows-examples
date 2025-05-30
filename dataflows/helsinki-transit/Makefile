BASE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../../)
include $(BASE_DIR)/Makefile

MQTT_VERSION=0.2.9
CDK_PKG_FILE := connector/infinyon-mqtt-source-$(MQTT_VERSION).ipkg
CONNECTOR_FILE=connector/mqtt-helsinki.yaml

CDK_SM=infinyon/jaq@0.1.0 

e2e:
	$(SDF_BIN) run --skip-running


download-sm:
	fluvio hub smartmodule download $(CDK_SM)

$(CDK_PKG_FILE):
	@echo "Downloading $@ …"
	cd connector; cdk hub download infinyon/mqtt-source@$(MQTT_VERSION)
	
.PHONY: cdk-download                   #
cdk-download: $(CDK_PKG_FILE)

cdk-start:	cdk-shutdown
	cdk deploy start --ipkg $(CDK_PKG_FILE) -c $(CONNECTOR_FILE)

cdk-shutdown:	cdk-download
	cdk deploy shutdown --name helsinki-mqtt || true
	
cdk-clean:	cdk-shutdown
	@echo "removing connector pkg: $(CDK_PKG_FILE)"
	rm $(CDK_PKG_FILE) || true


cloud-start:
	fluvio cloud connector create --config $(CONNECTOR_FILE)

cloud-stop:
	fluvio cloud connector delete helsinki-mqtt

