import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import {Account} from "./account.model"
import {AttestationMethod} from "./_attestationMethod"
import {ImplDeploymentPermission} from "./_implDeploymentPermission"
import {ImplBuild} from "./implBuild.model"
import {Worker} from "./worker.model"
import {Pool} from "./pool.model"

@Entity_()
export class Impl {
    constructor(props?: Partial<Impl>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Column_("int4", {nullable: false})
    implId!: number

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    _owner!: Account

    @Column_("text", {nullable: false})
    ownerAddress!: string

    @Column_("varchar", {length: 6, nullable: false})
    attestationMethod!: AttestationMethod

    @Column_("varchar", {length: 6, nullable: false})
    deploymentPermission!: ImplDeploymentPermission

    @Column_("int4", {nullable: false})
    oldestBuildVersion!: number

    @Column_("int4", {nullable: false})
    newestBuildVersion!: number

    @Column_("int4", {array: true, nullable: false})
    blockedBuildVersions!: (number)[]

    @Column_("text", {nullable: true})
    metadata!: string | undefined | null

    @Column_("int4", {nullable: false})
    poolsCount!: number

    @Column_("int4", {nullable: false})
    tasksCount!: number

    @Column_("int4", {nullable: false})
    onlineWorkersCount!: number

    @Column_("int4", {nullable: false})
    successfulTasksCount!: number

    @Column_("int4", {nullable: false})
    failedTasksCount!: number

    @Column_("int4", {nullable: false})
    erroredTasksCount!: number

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null

    @OneToMany_(() => ImplBuild, e => e._impl)
    builds!: ImplBuild[]

    @OneToMany_(() => Worker, e => e._impl)
    workers!: Worker[]

    @OneToMany_(() => Pool, e => e._impl)
    pools!: Pool[]
}