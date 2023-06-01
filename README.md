[![Build](https://github.com/endalk200/dockerify/actions/workflows/build.yml/badge.svg)](https://github.com/endalk200/dockerify/actions/workflows/build.yml) [![Release](https://github.com/endalk200/dockerify/actions/workflows/release.yml/badge.svg)](https://github.com/endalk200/dockerify/actions/workflows/release.yml)

# dockerify

A cli tool to generate dockerfile, docker-compose and kube manifest files for your project based on the language you are using and project configuration.

## Usage

To be able to use the cli tool just download the executable for your platform from the [release page](https://github.com/endalk200/dockerify/releases/tag/v0.0.5). Then you can run the executable from anywhere in your terminal.

You can also install the cli tool using the following command that downloads the executable and moves it to the bin directory for you to be able to run it from anywhere in your terminal:

**Linux**

```bash
# Download executable
curl -LJO https://github.com/endalk200/dockerify/releases/download//dockerify-v0.0.5-x86_64-linux.tar.xz

# Extract executable
tar -xvf dockerify-v0.0.5-x86_64-linux.tar.xz

# Move executable to bin
sudo mv dockerify-v0.0.5-x86_64-linux/dockerify /bin/

# Remove extracted directory
rm dockerify-v0.0.5-x86_64-linux.tar.xz
```

## Contributors

<a href="https://github.com/endalk200/dockerify/graphs/contributors">
  <p>
    <img width="60px" src="https://contrib.rocks/image?repo=endalk200/dockerify" alt="A table of avatars from the project's contributors" />
  </p>
</a>
