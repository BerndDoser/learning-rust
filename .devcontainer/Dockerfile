# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.241.1/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
 && apt-get -y install --no-install-recommends \
    libopenblas-dev

ENV LD_LIBRARY_PATH /usr/lib/x86_64-linux-gnu/openblas/

RUN wget https://github.com/quarto-dev/quarto-cli/releases/download/v1.0.36/quarto-1.0.36-linux-amd64.deb \
 && sudo apt install ./quarto-1.0.36-linux-amd64.deb

ARG USERNAME=vscode
ARG USER_UID=1026
ARG USER_GID=100

RUN usermod --uid $USER_UID --gid $USER_GID $USERNAME \
    && chown -R $USER_UID:$USER_GID /home/$USERNAME
