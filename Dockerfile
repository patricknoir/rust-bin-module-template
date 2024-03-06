FROM patricknoir/rust-base:latest as builder

COPY ./ /build
WORKDIR /build
RUN cargo build --release -p {{project-name}}

FROM ubuntu
RUN apt-get -y update
RUN apt-get install -y libpq-dev
RUN mkdir /account-service
COPY --from=builder /build/target/release/{{project-name}} /account-service/{{project-name}}
COPY --from=builder /build/{{project-name}}/.env /{{project-name}}/.env

CMD ["./{{project-name}}/{{project-name}}"]