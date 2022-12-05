# Makefile for RPI-Rover project
#
# Configuration:
# - REPO_PREFIX: 		Docker REPO_PREFIX prefix for project infra containers publishing.
# - BUILD_ID: 			Arbitrary build identifier, used for images tagging. Default: latest.
# - IMAGE_PLATFORM: 	Platform to target images to. Should match Raspberry Pi architecture.
#						Default: linux/arm/v7.
# - TARGET_PLATFORM: 	Platform to compile code for. Should match Raspberry Pi architecture.
# 						Default: arm-unknown-linux-gnueabihf
# - BUILDER_NAME: 		Docker BuildKit builder name for this build.
# - USE_CROSS: 			Define to use cross-rs for cross-compilation.
# 						Default: unset (do not use)
# - USE_CROSS_REMOTE:	Define if local docker client connects to remote docker server.
#						Default: unset (local docker server)
#
# Supported targets:
# - build: 			builds all parts of the project.
# - publish: 		publishes built docker containers of project infra to specified
# 					Docker $(REPO_PREFIX) with specified $(BUILD_ID) as tag.
# - deploy: 		deploys project infra into local docker using $(REPO_PREFIX) and
#					$(BUILD_ID) as tag.
# - undeploy: 		undeployed and removes docker containers from local docker that
# 					were previously spun up using 'deloy' target.
# - clean: 			attempts to clean all build artifacts, including any local docker
# 					images of project infra.
# Notes:
# - you should be logged in to the $(REPO_PREFIX) before trying to publish to it.
# - docker builds are done using buildkit (https://docs.docker.com/buildx/working-with-buildx/)
# 	which is expected to be installed beforehand.
#
# Examples:
# 	make deploy REPO_PREFIX=mydocker.home/myproject/ BUILD_ID=testbuild1
#

BUILD_ID					?= latest
BUILD_PROFILE 				?= release
IMAGE_PLATFORM				?= linux/arm/v7
TARGET_PLATFORM 			?= arm7-unknown-linux-gnueabihf

SUBPROJECTS 				= rover-firmware rover-infra
BUILD_TARGETS 				= $(SUBPROJECTS:%=build-%)
PUBLISH_TARGETS 			= $(SUBPROJECTS:%=publish-%)
DEPLOY_TARGETS				= $(SUBPROJECTS:%=deloy-%)
UNDEPLOY_TARGETS			= $(SUBPROJECTS:%=undeploy-%)
CLEAN_TARGETS 				= $(SUBPROJECTS:%=clean-%)

build: $(BUILD_TARGETS)

publish: $(PUBLISH_TARGETS)

deploy: $(DEPLOY_TARGETS)

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