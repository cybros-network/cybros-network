import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, OneToMany as OneToMany_} from "typeorm"
import {Worker} from "./worker.model"
import {Pool} from "./pool.model"
import {Task} from "./task.model"

@Entity_()
export class Account {
    constructor(props?: Partial<Account>) {
        Object.assign(this, props)
    }

    /**
     * Account address
     */
    @PrimaryColumn_()
    id!: string

    @Column_("int4", {nullable: false})
    workersCount!: number

    @Column_("int4", {nullable: false})
    poolsCount!: number

    @Column_("int4", {nullable: false})
    createdTasksCount!: number

    @OneToMany_(() => Worker, e => e._owner)
    owningWorkers!: Worker[]

    @OneToMany_(() => Pool, e => e._owner)
    owningPools!: Pool[]

    @OneToMany_(() => Task, e => e._owner)
    owningTasks!: Task[]
}
