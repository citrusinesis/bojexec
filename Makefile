BINARY_NAME=bojexec
BUILD_DIR=build
MAIN_PATH=cmd/bojexec/main.go

VERSION=$(shell git describe --tags --always --dirty)
COMMIT=$(shell git rev-parse HEAD)
BUILD_TIME=$(shell date -u '+%Y-%m-%d_%H:%M:%S')
PKG_PATH=main
LDFLAGS=-ldflags "-X ${PKG_PATH}.Version=${VERSION} -X ${PKG_PATH}.Commit=${COMMIT} -X ${PKG_PATH}.BuildTime=${BUILD_TIME}"

GOBASE=$(shell pwd)
GOBIN=$(GOBASE)/$(BUILD_DIR)

MAKEFLAGS += --silent

build:
	@echo "Building version ${VERSION}..."
	@echo "${LDFLAGS}"
	go build ${LDFLAGS} -o $(BUILD_DIR)/$(BINARY_NAME) $(MAIN_PATH)

run:
	go run $(MAIN_PATH)

clean:
	@echo "Cleaning..."
	rm -rf $(BUILD_DIR)
	go clean

test:
	go test ./... -v -count=1

test-coverage:
	go test ./... -v -count=1 -coverprofile=coverage.out
	go tool cover -html=coverage.out

lint:
	golangci-lint run

version:
	@echo "Version:    ${VERSION}"
	@echo "Commit:     ${COMMIT}"
	@echo "Build Time: ${BUILD_TIME}"

.PHONY: build run clean test test-coverage lint version