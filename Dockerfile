FROM patricknoir/rust-base:latest as builder

COPY ./ /build
WORKDIR /build
RUN cargo build --release -p {{project-name}}

FROM ubuntu
RUN apt-get -y update
RUN apt-get install -y libpq-dev
RUN mkdir /{{project-name}}
COPY --from=builder /build/target/release/{{project-name}} /{{project-name}}/{{project-name}}
COPY --from=builder /build/.env /{{project-name}}/.env
WORKDIR /{{project-name}}
CMD ["./{{project-name}}"]