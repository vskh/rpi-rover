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
	wasm-pack build --target web --no-typescript --out-dir app/pkg libux-web

publish: build

deploy: publish

undeploy:

clean:
	cross clean
	rm -rf libux-web/app/pkg
	rm -rf libux-web/target

.PHONY: build publish deploy undeploy clean

# end