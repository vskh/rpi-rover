# Makefile building containers of RPI-Rover projects.

REPOPREFIX 		?=
BUILDID			?= latest

BUILDPLATFORM	?= linux/arm/v6
BUILDERNAME		?= rpi-rover-builder-$(BUILDID)

CONTAINERS 		= rproxy wifiap
BUILD_TARGETS	= $(CONTAINERS:%=build-%)
PUBLISH_TARGETS	= $(CONTAINERS:%=publish-%)
CLEAN_TARGETS	= $(CONTAINERS:%=clean-%)

build: pre-build $(BUILD_TARGETS) post-build

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

$(BUILD_TARGETS): pre-build
	docker buildx build $(@:build-%=%) --platform=$(BUILDPLATFORM) --load -t $(REPOPREFIX)$(@:build-%=%):$(BUILDID)

post-build: $(BUILD_TARGETS)
	- docker buildx rm $(BUILDERNAME)

$(PUBLISH_TARGETS):
	docker push $(REPOPREFIX)$(@:publish-%=%):$(BUILDID)

$(CLEAN_TARGETS):
	docker images --filter="reference=$(@:clean-%=%):*" --format="{{.ID}}" | sort -u | xargs -r docker rmi --force

.PHONY: \
	pre-build build post-build $(BUILD_TARGETS) \
	pre-publish publish $(PUBLISH_TARGETS) \
	deploy undeploy \
	clean $(CLEAN_TARGETS)

# end