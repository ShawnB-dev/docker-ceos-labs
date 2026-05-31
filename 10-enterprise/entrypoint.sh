#!/bin/bash

# Start syslog-ng in the foreground so Docker can monitor the process
exec syslog-ng -F