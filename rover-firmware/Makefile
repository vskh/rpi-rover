# Makefile for Rust components of RPI-Rover project.

TARGETPLATFORM 		?= armv7-unknown-linux-gnueabihf
BUILDPROFILE 		?= --debug

export PATH 		:= $(HOME)/.cargo/bin:$(PATH)

build:
	cross build $(BUILDPROFILE) --target $(TARGETPLATFORM)

publish: build

deploy: publish

undeploy:

clean:
	cross clean

.PHONY: build publish deploy undeploy clean

# end