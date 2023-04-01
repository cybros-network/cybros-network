import * as dotenv from "dotenv"
dotenv.config()

import { type DataSource } from '@subsquid/substrate-processor'

const config: {
    dataSource: DataSource
} = {
    dataSource: {
        archive: process.env.ARCHIVE_GATEWAY_ENDPOINT || "http://localhost:8888/graphql",
        chain: process.env.CHAIN_NODE_RPC_ENDPOINT || "ws://localhost:9944",
    },
}

export default config
