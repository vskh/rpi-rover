# Makefile for RPI-Rover project
#
# Configuration:
# - REPOPREFIX: 	Docker REPOPREFIX prefix for project infra containers publishing.
# - BUILDID: 		Arbitrary build identifier, user for images tagging. Default: latest.
# - IMAGEPLATFORM: 	Platform to target images to. Should match Raspberry Pi architecture.
#					Default: linux/arm/v6.
# - TARGETPLATFORM: Platform to compile code for. Should match Raspberry Pi architecture.
# 					Default: arm-unknown-linux-gnueabihf
# - BUILDERNAME: 	Docker Buildkit builder name for this build.
#
# Supported targets:
# - build: 			builds all parts of the project.
# - publish: 		publishes built docker containers of project infra to specified
# 					Docker $(REPOPREFIX) with specified $(BUILDID) as tag.
# - deploy: 		deploys project infra into local docker using $(REPOPREFIX) and
#					$(BUILDID) as tag.
# - undeploy: 		undeployed and removes docker containers from local docker that
# 					were previously spun up using 'deloy' target.
# - clean: 			attempts to clean all build artifacts, including any local docker
# 					images of project infra.
# Notes:
# - you should be logged in to the $(REPOPREFIX) before trying to publish to it.
# - docker builds are done using buildkit (https://docs.docker.com/buildx/working-with-buildx/)
# 	which is expected to be installed beforehands.
#
# Examples:
# 	make deploy REPOPREFIX=mydocker.home/myproject/ BUILDID=testbuild1
#

BUILDID				?= latest
IMAGEPLATFORM		?= linux/arm/v6
TARGETPLATFORM 		?= arm-unknown-linux-gnueabihf

SUBPROJECTS 		= rover-api rover-infra
BUILD_TARGETS 		= $(SUBPROJECTS:%=build-%)
PUBLISH_TARGETS 	= publish-rover-infra
DEPLOY_TARGETS		= deploy-rover-infra
UNDEPLOY_TARGETS	= undeploy-rover-infra
CLEAN_TARGETS 		= $(SUBPROJECTS:%=clean-%)

build: $(BUILD_TARGETS)

publish: build $(PUBLISH_TARGETS)

deploy: publish $(DEPLOY_TARGETS)

undeploy: $(UNDEPLOY_TARGETS)

clean: $(CLEAN_TARGETS)

$(BUILD_TARGETS):
	$(MAKE) $(MAKE_FLAGS) -C $(@:build-%=%) build

$(PUBLISH_TARGETS):
	$(MAKE) $(MAKE_FLAGS) -C $(@:publish-%=%) publish

$(DEPLOY_TARGETS):
	$(MAKE) $(MAKE_FLAGS) -C $(@:deploy-%=%) deploy

$(UNDEPLOY_TARGETS):
	$(MAKE) $(MAKE_FLAGS) -C $(@:undeploy-%=%) undeploy

$(CLEAN_TARGETS):
	$(MAKE) $(MAKE_FLAGS) -C $(@:clean-%=%) clean

.PHONY: \
	build $(BUILD_TARGETS) \
	publish $(PUBLISH_TARGETS) \
	deploy $(DEPLOY_TARGETS) \
	undeploy $(UNDEPLOY_TARGETS) \
	clean $(CLEAN_TARGETS)

# end