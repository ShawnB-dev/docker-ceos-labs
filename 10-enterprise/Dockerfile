FROM rust:latest

# Install syslog-ng
RUN apt-get update && apt-get install -y \
    syslog-ng \
    && rm -rf /var/lib/apt/lists/*

# Create syslog-ng log directory (will be mounted by volume)
RUN mkdir -p /var/log/syslog

# Copy and make executable the entrypoint script
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]