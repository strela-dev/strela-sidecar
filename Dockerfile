FROM rust:latest as build
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

# create a new empty shell project
COPY . ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

# our final base
FROM alpine

# copy the build artifact from the build stage
COPY --from=build /usr/local/cargo/bin/strela-sidecar ./strela-sidecar

# set the startup command to run your binary
CMD ["./strela-sidecar"]