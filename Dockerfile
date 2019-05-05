FROM rustlang/rust:nightly

LABEL Maintainer="Kevin Gimbel"

WORKDIR /usr/src/fakedata_server
COPY . .

RUN cargo install --path .

EXPOSE 8000

CMD ["fakedata_server"]