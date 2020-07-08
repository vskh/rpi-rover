# Makefile for Rust components of RPI-Rover project.

TARGETPLATFORM 		?= armv7-unknown-linux-gnueabihf
BUILDPROFILE 		?= release

export PATH 		:= $(HOME)/.cargo/bin:$(PATH)

ifeq ($(BUILDPROFILE),release)
	PROFILEOPT := --release
else
	PROFILEOPT :=
endif

build:
	cross build $(PROFILEOPT) --target $(TARGETPLATFORM)

publish: build

deploy: publish

undeploy:

clean:
	cross clean

.PHONY: build publish deploy undeploy clean

# end