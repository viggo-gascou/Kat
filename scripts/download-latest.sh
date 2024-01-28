#!/bin/bash

# Get the latest release
latest_release=$(curl --silent "https://api.github.com/repos/viggo-gascou/releases/latest")

# Get the tag name of the latest release assuming semvar format (vX.Y.Z)
tag_name=$(echo $latest_release | grep -Po '"tag_name": "\Kv[0-9]+\.[0-9]+\.[0-9]+(?=")')

# Construct the filename
filename="kat-${tag_name}-$(uname)-$(uname -m)"

# Download the file
curl -L -O "https://github.com/viggo-gascou/releases/latest/download/${filename}" -o "kat"