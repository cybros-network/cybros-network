import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_} from "typeorm"
import * as marshal from "./marshal"
import {CreatingTaskPermission} from "./_creatingTaskPermission"

@Entity_()
export class CreatingTaskPolicy {
    constructor(props?: Partial<CreatingTaskPolicy>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Column_("varchar", {length: 6, nullable: false})
    permission!: CreatingTaskPermission

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: true})
    price!: bigint | undefined | null

    @Column_("int4", {nullable: true})
    startBlock!: number | undefined | null

    @Column_("int4", {nullable: true})
    endBlock!: number | undefined | null
}
