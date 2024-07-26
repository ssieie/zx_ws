FROM ubuntu:20.04
RUN apt-get update && apt-get upgrade -y

RUN apt-get install -y -q build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /zx_blog_server
COPY ./webservice/. /zx_blog_server/webservice/
COPY .env /zx_blog_server/
COPY Cargo.toml /zx_blog_server/
COPY Cargo.lock /zx_blog_server/
RUN cargo build --release

EXPOSE 9000
ENTRYPOINT /zx_blog_server/target/release/blog-service