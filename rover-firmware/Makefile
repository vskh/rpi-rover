# Makefile for Rust components of RPI-Rover project.

TARGET_PLATFORM 	?= armv7-unknown-linux-gnueabihf
BUILD_PROFILE 		?= release

export PATH 		:= $(HOME)/.cargo/bin:$(PATH)

ifeq ($(BUILD_PROFILE),release)
	PROFILEOPT 		:= --release
else
	PROFILEOPT 		:=
endif

BUILD_TOOL 			:= cargo
ifdef USE_CROSS
	BUILD_TOOL 		:= cross
	ifdef USE_CROSS_REMOTE
		BUILD_TOOL	:= CROSS_REMOTE=1 cross
	endif
endif

build:
	$(BUILD_TOOL) build $(PROFILEOPT) --target $(TARGET_PLATFORM)
	trunk build --release libux-web/index.html

publish: build

deploy: publish

undeploy:

clean:
	cross clean
	rm -rf libux-web/dist
	rm -rf libux-web/target

.PHONY: build publish deploy undeploy clean

# end