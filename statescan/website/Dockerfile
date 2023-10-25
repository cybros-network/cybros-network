FROM node:20-alpine

RUN apk update && apk add --no-cache build-base git
RUN npm install pm2 -g

ARG GIT_REPO="https://github.com/opensquare-network/statescan-v2.git"
ARG GIT_TAG="main"
RUN git clone --depth 1 --recurse-submodules --shallow-submodules -j 8 -b ${GIT_TAG} ${GIT_REPO} /opt/statescan

WORKDIR /opt/statescan/site

RUN yarn

ADD ecosystem.config.js ./
ADD ./src ./src

# ENV PUBLIC_IDENTITY_SERVER_HOST=""
ENV PUBLIC_API_END_POINT="http://127.0.0.1:5010/"
ENV PUBLIC_IDENTITY_API_END_POINT=""
ENV SUB_CHAIN_NAME="cybros-primal"
ENV DEFAULT_IPFS_GATEWAY=""

CMD ["pm2-runtime", "ecosystem.config.js"]
