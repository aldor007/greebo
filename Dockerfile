FROM rust

WORKDIR /usr/src/greebo
COPY . .

RUN carbo build --release

CMD ["/usr/src/greebo/greebo"]