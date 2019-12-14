# Makefile building containers of RPI-Rover projects.

REPOPREFIX 			?=
BUILDID				?= latest

BUILDPLATFORM		?= linux/arm/v6
BUILDERNAME			?= rpi-rover-builder-$(BUILDID)

CONTAINERS 			= rproxy wifiap servoblaster
PREBUILD_TARGETS	= prebuild-servoblaster
BUILD_TARGETS		= $(CONTAINERS:%=build-%)
PUBLISH_TARGETS		= $(CONTAINERS:%=publish-%)
CLEAN_TARGETS		= $(CONTAINERS:%=clean-%)

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

$(BUILD_TARGETS): pre-build $(PREBUILD_TARGETS)
	docker buildx build $(@:build-%=%) --platform=$(BUILDPLATFORM) --load -t $(REPOPREFIX)$(@:build-%=%):$(BUILDID)

post-build:
	- docker buildx rm $(BUILDERNAME)

$(PUBLISH_TARGETS):
	docker push $(REPOPREFIX)$(@:publish-%=%):$(BUILDID)

clean-servoblaster-extra:
	rm -rf servoblaster/src

$(CLEAN_TARGETS): clean-servoblaster-extra
	docker images --filter="reference=$(@:clean-%=%):*" --format="{{.ID}}" | sort -u | xargs -r docker rmi --force

.PHONY: \
	pre-build $(PREBUILD_TARGETS) build post-build $(BUILD_TARGETS) \
	pre-publish publish $(PUBLISH_TARGETS) \
	deploy undeploy \
	clean-servoblaster-extra $(CLEAN_TARGETS) clean

# end