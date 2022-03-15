FROM gitpod/workspace-full

RUN bash -cl "rustup target add wasm32-unknown-unknown"

RUN bash -c ". .nvm/nvm.sh \
             && nvm install v12 && nvm alias default v12 && nvm use default && npm install -g yarn"