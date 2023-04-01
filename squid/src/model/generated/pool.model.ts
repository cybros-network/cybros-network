import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import * as marshal from "./marshal"
import {Account} from "./account.model"
import {WorkersPools} from "./workersPools.model"
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

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    ownerDeposit!: bigint

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    stashAccount!: Account

    @Column_("bool", {nullable: false})
    creatingTaskAbility!: boolean

    @Column_("int4", {nullable: false})
    creatingTaskPoliciesCount!: number

    @Column_("int4", {nullable: false})
    tasksCount!: number

    @Column_("int4", {nullable: false})
    workersCount!: number

    @OneToMany_(() => WorkersPools, e => e.pool)
    workers!: WorkersPools[]

    @OneToMany_(() => Task, e => e.pool)
    tasks!: Task[]
}
