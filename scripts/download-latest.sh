#!/bin/bash

# Get the latest release
latest_release=$(curl --silent "https://api.github.com/repos/viggo-gascou/kat-rs/releases/latest")

# Get the tag name of the latest release assuming semvar format (vX.Y.Z)
tag_name=$(echo $latest_release | sed -n 's/.*"tag_name": "\(v[0-9]*\.[0-9]*\.[0-9]*\)",.*/\1/p')

# Construct the filename
filename="kat-${tag_name}-$(uname)-$(uname -m)"
echo https://github.com/viggo-gascou/kat-rs/releases/download/${tag_name}/${filename}

# Download the file
curl -L "https://github.com/viggo-gascou/kat-rs/releases/download/${tag_name}/${filename}" -o kat-$(uname)-$(uname -m)