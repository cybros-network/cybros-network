const chain = process.env.SUB_CHAIN_NAME ?? "cybros-prime"
const apiHost = process.env.PUBLIC_API_END_POINT ?? "http://127.0.0.1:5010/"
// Optional API for identity pallet
const identityApiEndpoint = process.env.PUBLIC_IDENTITY_API_END_POINT ?? ""
// Uses for unique pallet, set it blank for safety.
const ipfsGateway = process.env.IPFS_GATEWAY ?? ""
// Not in use, set it blank for safety.
const identityServerHost = process.env.PUBLIC_IDENTITY_SERVER_HOST ?? ""

const env = {
  "NODE_ENV": "production",
  "LOG_LEVEL": "info",
  "REACT_APP_PUBLIC_CHAIN": chain,
  "REACT_APP_PUBLIC_IDENTITY_SERVER_HOST": identityServerHost,
  "REACT_APP_PUBLIC_API_END_POINT": apiHost,
  "REACT_APP_PUBLIC_IDENTITY_API_END_POINT": identityApiEndpoint,
  "REACT_APP_DEFAULT_IPFS_GATEWAY": ipfsGateway,
}

module.exports = {
  apps : [
    {
      name: "website",
      script: "yarn",
      interpreter: "none",
      args: "start -s build -p 3000",
      watch: true,
      env
    },
  ],
};
