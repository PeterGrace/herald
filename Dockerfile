FROM debian:stretch-slim
ENV TINI_URL="https://github.com/krallin/tini/releases/download/v0.18.0/tini-static-amd64"
ENV PACKAGES="libssl1.0 ca-certificates"

RUN apt-get -y update \
 && apt-get -y install ${PACKAGES}

ADD ${TINI_URL} /tini
RUN chmod a+x /tini \
    && mkdir -p /herald

ENTRYPOINT ["/tini", "--"]

COPY target/release/herald /herald/herald
ADD docker/entrypoint.sh /entrypoint.sh
CMD ["/entrypoint.sh"]

