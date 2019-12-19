# Makefile for Rust components of RPI-Rover project.

TARGETPLATFORM 		?= armv7-unknown-linux-gnueabihf
BUILDPROFILE 		?= --release

export PATH 		:= $(HOME)/.cargo/bin:$(PATH)

build:
	cross build $(BUILDPROFILE) --target $(TARGETPLATFORM)

clean:
	cross clean

.PHONY: build clean

# end