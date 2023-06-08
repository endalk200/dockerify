#!/bin/bash

if [ -z "$1" ]; then
    echo "Error: Version number not provided"
    exit 1
fi

version=$1

echo "Updating version in README to $version"

sed -i "s/v[0-9].[0-9].[0-9]/v$version/g" README.md
