import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import {Account} from "./account.model"
import {WorkersPools} from "./workersPools.model"
import {CreatingTaskPolicy} from "./creatingTaskPolicy.model"
import {Task} from "./task.model"

@Entity_()
export class Pool {
    constructor(props?: Partial<Pool>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    owner!: Account

    @Column_("bool", {nullable: false})
    creatingTaskAbility!: boolean

    @Column_("int4", {nullable: false})
    workersCount!: number

    @Column_("int4", {nullable: false})
    creatingTaskPoliciesCount!: number

    @Column_("int4", {nullable: false})
    tasksCount!: number

    @Column_("bytea", {nullable: true})
    metadata!: Uint8Array | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null

    @OneToMany_(() => WorkersPools, e => e.pool)
    workers!: WorkersPools[]

    @OneToMany_(() => CreatingTaskPolicy, e => e.pool)
    creatingTaskPolicies!: CreatingTaskPolicy[]

    @OneToMany_(() => Task, e => e.pool)
    tasks!: Task[]
}
