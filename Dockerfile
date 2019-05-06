FROM clux/muslrust:nightly as builder
WORKDIR /volume
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/fakedata_server .
ENTRYPOINT [ "/fakedata_server" ]