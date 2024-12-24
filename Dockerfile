ARG RUST_VERSION=1.78.0

FROM instrumentisto/rust:beta

WORKDIR /usr/src/text_classifier_extractor
COPY . .

RUN cargo install --path .

CMD ["text_classifier_extractor"]