tag := $(shell git describe)

install:
	cargo install


format:
	@(carrgo  fmt)


docker-push:
	docker build -t aldor007/greebo -f Dockerfile . -t aldor007/greebo:latest; docker push aldor007/greebo:latest

