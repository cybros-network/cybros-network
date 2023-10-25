Cybros StateScan deployment
====

## Prepare

The repo now points to the Cybros dev network.
To change to another network:

- `cp .env.example .env`
- Review and edit `.env`
  - If the deployment intents to expose to the Internet, `BACKEND_API_HOST` should be the public domain
- Review and edit `backend/packages/server/src/utils/consts/chains.js`
- Review and edit `website/src/utils/consts/chains/index.js`
  - Do not change `identity: "polkadot",`
- Comment `website` section in `docker-compose.yml` if we want to deploy the frontend in another place

## Run

`docker compose up`

## Use

By default, backend API use the port `5010`, and website use the port `3000`.
If the deployment intents to expose to the Internet, you should reverse proxying these ports.

## TODO

- Wait `scan-meta` open source
  - Current indexing is slow
