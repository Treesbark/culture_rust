# docker build -f Dockerfile -t culture_rust .
# docker run -p 8090:8090 -t culture_rust  
# docker tag culture_rust treesbark/culture_rust:simple
# docker push treesbark/culture_rust:simple  

FROM arm32v7/rust:1.57 as build

# create a new empty shell project
RUN USER=root cargo new --bin culture_rust
WORKDIR /culture_rust

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Rocket.toml ./Rocket.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/culture_rust*
RUN cargo build --release

# our final base
FROM debian:buster-slim 

# copy the build artifact from the build stage
COPY --from=build /culture_rust/target/release/culture_rust .
COPY ./Rocket.toml .

EXPOSE 8090

# set the startup command to run your binary
CMD ["./culture_rust"]