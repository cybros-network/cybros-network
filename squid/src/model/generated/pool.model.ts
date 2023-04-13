import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_, OneToMany as OneToMany_} from "typeorm"
import {Account} from "./account.model"
import {Impl} from "./impl.model"
import {PoolWorkers} from "./poolWorkers.model"
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

    @Index_()
    @ManyToOne_(() => Impl, {nullable: true})
    impl!: Impl

    @Column_("bool", {nullable: false})
    creatingTaskAbility!: boolean

    @Column_("text", {nullable: true})
    metadata!: string | undefined | null

    @Column_("timestamp with time zone", {nullable: false})
    createdAt!: Date

    @Column_("timestamp with time zone", {nullable: false})
    updatedAt!: Date

    @Column_("timestamp with time zone", {nullable: true})
    deletedAt!: Date | undefined | null

    @OneToMany_(() => PoolWorkers, e => e.pool)
    workers!: PoolWorkers[]

    @OneToMany_(() => CreatingTaskPolicy, e => e.pool)
    creatingTaskPolicies!: CreatingTaskPolicy[]

    @OneToMany_(() => Task, e => e.pool)
    tasks!: Task[]
}
