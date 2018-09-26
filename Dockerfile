FROM rust

WORKDIR /usr/src/greebo
COPY . .

RUN cargo install

CMD ["greebo"]