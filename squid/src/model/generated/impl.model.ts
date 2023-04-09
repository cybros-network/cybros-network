import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import {Account} from "./account.model"
import {AttestationMethod} from "./_attestationMethod"
import {ImplDeploymentPermission} from "./_implDeploymentPermission"
import {Worker} from "./worker.model"
import {Pool} from "./pool.model"

@Entity_()
export class Impl {
    constructor(props?: Partial<Impl>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    owner!: Account

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

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null

    @OneToMany_(() => Worker, e => e.impl)
    workers!: Worker[]

    @OneToMany_(() => Pool, e => e.impl)
    pools!: Pool[]
}
