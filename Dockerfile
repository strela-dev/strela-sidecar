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

RUN apk add --no-cache curl

# Download kubectl and make it executable
RUN curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl" \
    && chmod +x ./kubectl \
    && mv ./kubectl /usr/local/bin/kubectl

ENV RUST_LOG=info

# set the startup command to run your binary
CMD ["./strela-sidecar"]