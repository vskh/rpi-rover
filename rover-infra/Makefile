# Makefile building containers of RPI-Rover projects.

REPO_PREFIX 		?=
BUILD_ID			?= latest

IMAGE_PLATFORM		?= linux/arm/v7
TARGET_PLATFORM 	?= armv7-unknown-linux-gnueabihf
BUILD_PROFILE 		?= release
BUILDER_NAME		?= rpi-rover-builder-$(BUILD_ID)

CONTAINERS 			= wifiap servoblaster frontend rover-api-net rover-api-http rover-ux-console # video
PREBUILD_TARGETS	= prebuild-frontend prebuild-servoblaster prebuild-rover-api-net prebuild-rover-api-http prebuild-rover-ux-console
BUILD_TARGETS		= $(CONTAINERS:%=build-%)
PUBLISH_TARGETS		= $(CONTAINERS:%=publish-%)
CLEAN_TARGETS		= $(CONTAINERS:%=clean-%)
CLEAN_EXTRA_TARGETS = clean-frontend-extra clean-servoblaster-extra clean-rover-api-net-extra clean-rover-api-http-extra clean-rover-ux-console-extra

build: $(BUILD_TARGETS) post-build

publish: build $(PUBLISH_TARGETS)

pre-publish:
ifndef REPO_PREFIX
	$(error REPO_PREFIX is not set. Required for publishing.)
endif

deploy: publish
	TAG=$(BUILD_ID) REPO_PREFIX=$(REPO_PREFIX) docker-compose -f docker-compose-deploy.yml up -d

undeploy:
	TAG=$(BUILD_ID) REPO_PREFIX=$(REPO_PREFIX) docker-compose -f docker-compose-deploy.yml down

clean: $(CLEAN_TARGETS)

pre-build:
	- docker buildx create --name $(BUILDER_NAME)
	docker buildx inspect --bootstrap $(BUILDER_NAME)
	docker buildx use $(BUILDER_NAME)

frontend/data/app:
	cp -r ../rover-firmware/libux-web/dist frontend/data

prebuild-frontend: frontend/data/dist

servoblaster/src:
	cp -r ../3rdparty/PiBits/ServoBlaster/user servoblaster/src

prebuild-servoblaster: servoblaster/src

rover-api-net/app:
	mkdir -p rover-api-net/app
	cp -v ../rover-firmware/api-net/Config.toml rover-api-net/app
	cp -v ../rover-firmware/api-net/log4rs.yml rover-api-net/app
	cp -v ../rover-firmware/target/$(TARGET_PLATFORM)/$(BUILD_PROFILE)/api-net rover-api-net/app

prebuild-rover-api-net: rover-api-net/app

rover-api-http/app:
	mkdir -p rover-api-http/app
	cp -v ../rover-firmware/api-http/Config.toml rover-api-http/app
	cp -v ../rover-firmware/api-http/log4rs.yml rover-api-http/app
	cp -v ../rover-firmware/target/$(TARGET_PLATFORM)/$(BUILD_PROFILE)/api-http rover-api-http/app

prebuild-rover-api-http: rover-api-http/app

rover-ux-console/app:
	mkdir -p rover-ux-console/app
	cp -v ../rover-firmware/target/$(TARGET_PLATFORM)/$(BUILD_PROFILE)/ux-console rover-ux-console/app

prebuild-rover-ux-console: rover-ux-console/app

$(BUILD_TARGETS): pre-build $(PREBUILD_TARGETS)
	docker buildx build $(@:build-%=%) \
		--platform=$(IMAGE_PLATFORM) \
		--load \
		-t $(REPO_PREFIX)$(@:build-%=%):$(BUILD_ID) \
		--build-arg TARGET_PLATFORM=$(TARGET_PLATFORM)

post-build:
	- docker buildx rm $(BUILDER_NAME)

$(PUBLISH_TARGETS): pre-publish
	docker push $(REPO_PREFIX)$(@:publish-%=%):$(BUILD_ID)

clean-frontend-extra:
	rm -rf frontend/data/app

clean-servoblaster-extra:
	rm -rf servoblaster/src

clean-rover-api-http-extra:
	rm -rf rover-api-http/app

clean-rover-api-net-extra:
	rm -rf rover-api-net/app

clean-rover-ux-console-extra:
	rm -rf rover-ux-console/app

$(CLEAN_TARGETS): $(CLEAN_EXTRA_TARGETS) post-build
	docker images --filter="reference=$(@:clean-%=%):*" --format="{{.ID}}" | sort -u | xargs -r docker rmi --force

.PHONY: \
	pre-build $(PREBUILD_TARGETS) build post-build $(BUILD_TARGETS) \
	pre-publish publish $(PUBLISH_TARGETS) \
	deploy undeploy \
	$(CLEAN_EXTRA_TARGETS) $(CLEAN_TARGETS) clean

# end
