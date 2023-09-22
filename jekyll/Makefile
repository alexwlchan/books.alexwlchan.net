export DOCKER_IMAGE_NAME = ghcr.io/alexwlchan/alexwlchan.net
export DOCKER_IMAGE_VERSION = 42
DOCKER_IMAGE = $(DOCKER_IMAGE_NAME):$(DOCKER_IMAGE_VERSION)

ROOT = $(shell git rev-parse --show-toplevel)/jekyll

SERVER_PORT = 5959

publish-docker:
	ruby scripts/publish_docker_image.rb

html:
	docker run --tty --rm \
		--volume /var/run/docker.sock:/var/run/docker.sock \
		--volume $(ROOT):$(ROOT) \
		--workdir $(ROOT) \
		$(DOCKER_IMAGE) build --trace

html-drafts:
	docker run --tty --rm \
		--volume /var/run/docker.sock:/var/run/docker.sock \
		--volume $(ROOT):$(ROOT) \
		--workdir $(ROOT) \
		$(DOCKER_IMAGE) build --trace --drafts

serve:
	docker run --tty --rm \
		--volume /var/run/docker.sock:/var/run/docker.sock \
		--volume $(ROOT):$(ROOT) \
		--workdir $(ROOT) \
		--publish $(SERVER_PORT):$(SERVER_PORT) \
		$(DOCKER_IMAGE) serve \
			--drafts \
			--incremental \
			--host "0.0.0.0" \
			--port $(SERVER_PORT) \
			--skip-initial-build \
			--trace

deploy:
	docker run --tty --rm \
		--volume $(ROOT):$(ROOT) \
		--workdir $(ROOT) \
		ghcr.io/williamjacksn/netlify-cli \
		deploy --auth "$(NETLIFY_AUTH_TOKEN)"

deploy-prod:
	docker run --tty --rm \
		--volume $(ROOT):$(ROOT) \
		--workdir $(ROOT) \
		ghcr.io/williamjacksn/netlify-cli \
		deploy --prod --auth "$(NETLIFY_AUTH_TOKEN)"

Gemfile.lock: Gemfile
	docker run \
		--volume $(ROOT):$(ROOT) \
		--workdir $(ROOT) \
		--tty --rm $(shell cat Dockerfile | grep FROM | awk '{print $$2}') \
		bundle lock --update
