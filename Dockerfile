FROM debian:buster

# Get certificates.
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates

# Copy executable
WORKDIR /app
COPY target/release/sistem_projects projects

# Entry point
ENTRYPOINT ["./projects"]
