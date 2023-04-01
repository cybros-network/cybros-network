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

    @OneToMany_(() => Worker, e => e.owner)
    owningWorkers!: Worker[]

    @OneToMany_(() => Pool, e => e.owner)
    owningPools!: Pool[]

    @OneToMany_(() => Task, e => e.owner)
    owningTasks!: Task[]
}
