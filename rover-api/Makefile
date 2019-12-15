# Makefile for Rust components of RPI-Rover project.

TARGETPLATFORM 		?= arm-unknown-linux-gnueabihf
BUILDPROFILE 		?= --release

export PATH 		:= $(HOME)/.cargo/bin:$(PATH)

build:
	cross build $(BUILDPROFILE) --target $(TARGETPLATFORM)

clean:
	cross clean

.PHONY: build clean

# end