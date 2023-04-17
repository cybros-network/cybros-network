import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import {Account} from "./account.model"
import {Impl} from "./impl.model"
import {ImplBuild} from "./implBuild.model"
import {WorkerStatus} from "./_workerStatus"
import {AttestationMethod} from "./_attestationMethod"
import {OfflineReason} from "./_offlineReason"
import {WorkerEvent} from "./workerEvent.model"
import {PoolWorkers} from "./poolWorkers.model"
import {Task} from "./task.model"

@Entity_()
export class Worker {
    constructor(props?: Partial<Worker>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    _owner!: Account

    @Column_("text", {nullable: false})
    ownerAddress!: string

    @Index_()
    @ManyToOne_(() => Impl, {nullable: true})
    _impl!: Impl | undefined | null

    @Column_("int4", {nullable: true})
    implId!: number | undefined | null

    @Index_()
    @ManyToOne_(() => ImplBuild, {nullable: true})
    _implBuild!: ImplBuild | undefined | null

    @Column_("int4", {nullable: true})
    implBuildVersion!: number | undefined | null

    @Index_()
    @Column_("varchar", {length: 17, nullable: false})
    status!: WorkerStatus

    @Column_("int4", {nullable: false})
    implSpecVersion!: number

    @Column_("varchar", {length: 6, nullable: true})
    attestationMethod!: AttestationMethod | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    attestationExpiresAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    lastAttestedAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    lastHeartbeatReceivedAt!: Date | undefined | null

    @Column_("timestamp with time zone", {nullable: true})
    offlineAt!: Date | undefined | null

    @Column_("varchar", {length: 24, nullable: true})
    offlineReason!: OfflineReason | undefined | null

    @Column_("int4", {nullable: false})
    poolsCount!: number

    @Column_("int4", {nullable: false})
    inProcessingTasksCount!: number

    @Column_("int4", {nullable: false})
    assignedTasksCount!: number

    @Column_("int4", {nullable: false})
    successfulTasksCount!: number

    @Column_("int4", {nullable: false})
    failedTasksCount!: number

    @Column_("int4", {nullable: false})
    erroredTasksCount!: number

    @Column_("int4", {nullable: false})
    uptime!: number

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null

    @OneToMany_(() => WorkerEvent, e => e._worker)
    events!: WorkerEvent[]

    @OneToMany_(() => PoolWorkers, e => e._worker)
    servingPools!: PoolWorkers[]

    @OneToMany_(() => Task, e => e._assignee)
    assignedTasks!: Task[]
}
