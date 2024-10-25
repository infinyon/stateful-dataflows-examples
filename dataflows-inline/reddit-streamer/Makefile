# Makefile
.PHONY: clean rebuild

#edit these with keywords to watch separated by ,
#spaces may brake not sure 
WATCHING_VAR ?= rust,fluvio,infinyon
SUBS ?= rust,test,
TOKEN ?=
NEW_CONNECTOR ?= 

clean:
	@echo "Cleaning up..."
	@echo "Cleaning SDF..."
	sdf clean -f
	@echo "Cleaning connectors..."
	./cleancon
	rm *.log
	rm -r rust
	rm -r conn

sdf:
	@echo "Building SDF..."
	sdf run --ui -e watching=$(WATCHING_VAR)

connectors:
	@echo "Building building connectors..."
	./generator $(SUBS)

test-conn: 
	@echo "Testing connector(s)..."
	fluvio consume reddit-sub-posts -T 3 -d

rebuild: clean build
build: connectors sdf
