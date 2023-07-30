import {
    BlockHeader,
    DataHandlerContext,
    SubstrateBatchProcessor,
    SubstrateBatchProcessorFields,
} from "@subsquid/substrate-processor"
import { Store } from '@subsquid/typeorm-store'

import config from "./config"

export const processor = new SubstrateBatchProcessor()
    .setDataSource(config.dataSource)
    .setFields({
        block: {
            timestamp: true
        },
        extrinsic: {
            error: true,
            success: true,
            hash: true
        },
        call: {
            origin: true,
            name: true,
            args: true
        },
        event: {
            name: true,
            args: true
        }
    })
    .setBlockRange({
      from: 1
    })
    .addEvent({
        name: [
            // OffchainComputingWorkers
            "OffchainComputingWorkers.WorkerRegistered",
            "OffchainComputingWorkers.WorkerDeregistered",
            "OffchainComputingWorkers.WorkerOnline",
            "OffchainComputingWorkers.WorkerUnresponsive",
            "OffchainComputingWorkers.WorkerRequestingOffline",
            "OffchainComputingWorkers.WorkerOffline",
            "OffchainComputingWorkers.WorkerHeartbeatReceived",
            "OffchainComputingWorkers.WorkerAttestationRefreshed",
            "OffchainComputingWorkers.ImplRegistered",
            "OffchainComputingWorkers.ImplDeregistered",
            "OffchainComputingWorkers.ImplDeploymentScopeUpdated",
            "OffchainComputingWorkers.ImplMetadataUpdated",
            "OffchainComputingWorkers.ImplMetadataRemoved",
            "OffchainComputingWorkers.ImplBuildRegistered",
            "OffchainComputingWorkers.ImplBuildDeregistered",
            "OffchainComputingWorkers.ImplBuildStatusUpdated",
            // OffchainComputing
            "OffchainComputing.PoolCreated",
            "OffchainComputing.PoolDestroyed",
            "OffchainComputing.PoolMetadataUpdated",
            "OffchainComputing.PoolMetadataRemoved",
            "OffchainComputing.PoolSettingsUpdated",
            "OffchainComputing.JobPolicyCreated",
            "OffchainComputing.JobPolicyDestroyed",
            "OffchainComputing.JobPolicyEnablementUpdated",
            "OffchainComputing.WorkerProvisioned",
            "OffchainComputing.WorkerRevoked",
            "OffchainComputing.WorkerSubscribed",
            "OffchainComputing.WorkerUnsubscribed",
            "OffchainComputing.JobCreated",
            "OffchainComputing.JobDestroyed",
            "OffchainComputing.JobAssigned",
            "OffchainComputing.JobReleased",
            "OffchainComputing.JobStatusUpdated",
            "OffchainComputing.JobResultUpdated"
        ]
    })

export type Fields = SubstrateBatchProcessorFields<typeof processor>
export type Context = DataHandlerContext<Store, Fields>
export type Block = BlockHeader<Fields>
