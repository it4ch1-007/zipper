# Use a base image with Node.js and Rust installed
FROM rust:latest
FROM node:16


RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash \
    && export NVM_DIR="$HOME/.nvm" \
    && [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" \
    && nvm install 16

ENV NVM_DIR /root/.nvm
ENV NODE_VERSION 16

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install -y nodejs


# Install yarn (if not already installed)


# Set the working directory inside the container
WORKDIR /app

# Copy the entire project directory into the container
COPY . .

# Install Tauri CLI globally
RUN yarn global add @tauri-apps/cli \
    yarn global add @tauri-apps/cli-linux-x64-gnu

# Navigate to the Tauri project directory and install dependencies
WORKDIR /app/src-tauri/src
RUN rm -f package-lock.json
RUN yarn cache clean --force


#Installing other dependencies
RUN yarn global add @tauri-apps/api

# Set the working directory back to the root of your project
WORKDIR /app

# Expose necessary ports if required
# EXPOSE <port>

# Set the startup command for the container
CMD ["yarn", "tauri", "dev"]
