# Makefile building containers of RPI-Rover projects.

REPOPREFIX 			?=
BUILDID				?= latest

IMAGEPLATFORM		?= linux/arm/v6
TARGETPLATFORM 		?= arm-unknown-linux-gnueabihf
BUILDERNAME			?= rpi-rover-builder-$(BUILDID)

CONTAINERS 			= rproxy wifiap servoblaster rover-api
PREBUILD_TARGETS	= prebuild-servoblaster prebuild-rover-api
BUILD_TARGETS		= $(CONTAINERS:%=build-%)
PUBLISH_TARGETS		= $(CONTAINERS:%=publish-%)
CLEAN_TARGETS		= $(CONTAINERS:%=clean-%)
CLEAN_EXTRA_TARGETS = clean-servoblaster-extra clean-rover-api-extra

build: pre-build $(PREBUILD_TARGETS) $(BUILD_TARGETS) post-build

publish: build pre-publish $(PUBLISH_TARGETS)

pre-publish:
ifndef REPOPREFIX
	$(error REPOPREFIX is not set. Required for publishing.)
endif

deploy: publish
	TAG=$(BUILDID) REPOPREFIX=$(REPOPREFIX) docker-compose -f docker-compose-deploy.yml up -d

undeploy:
	TAG=$(BUILDID) REPOPREFIX=$(REPOPREFIX) docker-compose -f docker-compose-deploy.yml down

clean: $(CLEAN_TARGETS)

pre-build:
	- docker buildx create --name $(BUILDERNAME)
	docker buildx inspect --bootstrap $(BUILDERNAME)
	docker buildx use $(BUILDERNAME)

servoblaster/src:
	cp -r ../3rdparty/PiBits/ServoBlaster/user servoblaster/src

prebuild-servoblaster: servoblaster/src

rover-api/target:
	cp -r ../rover-api/target rover-api/target

prebuild-rover-api: rover-api/target

$(BUILD_TARGETS): pre-build $(PREBUILD_TARGETS)
	docker buildx build $(@:build-%=%) \
		--platform=$(IMAGEPLATFORM) \
		--load \
		-t $(REPOPREFIX)$(@:build-%=%):$(BUILDID) \
		--build-arg TARGETPLATFORM=$(TARGETPLATFORM)

post-build:
	- docker buildx rm $(BUILDERNAME)

$(PUBLISH_TARGETS):
	docker push $(REPOPREFIX)$(@:publish-%=%):$(BUILDID)

clean-servoblaster-extra:
	rm -rf servoblaster/src

clean-rover-api-extra:
	rm -rf rover-api/src

$(CLEAN_TARGETS): $(CLEAN_EXTRA_TARGETS) post-build
	docker images --filter="reference=$(@:clean-%=%):*" --format="{{.ID}}" | sort -u | xargs -r docker rmi --force

.PHONY: \
	pre-build $(PREBUILD_TARGETS) build post-build $(BUILD_TARGETS) \
	pre-publish publish $(PUBLISH_TARGETS) \
	deploy undeploy \
	$(CLEAN_EXTRA_TARGETS) $(CLEAN_TARGETS) clean

# end