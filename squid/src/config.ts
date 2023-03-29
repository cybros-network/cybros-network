import * as dotenv from "dotenv"
dotenv.config()

export const chainNodeRPCEndpoint = process.env.CHAIN_NODE_RPC_ENDPOINT || "wss://node-rpc.cybros.network"

export const archiveGatewayEndpoint = process.env.ARCHIVE_GATEWAY_ENDPOINT || "http://localhost:8888/graphql"
