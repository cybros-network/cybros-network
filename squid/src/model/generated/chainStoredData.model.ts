import {Entity as Entity_, Column as Column_, PrimaryColumn as PrimaryColumn_, ManyToOne as ManyToOne_, Index as Index_} from "typeorm"
import * as marshal from "./marshal"
import {Account} from "./account.model"

@Entity_()
export class ChainStoredData {
    constructor(props?: Partial<ChainStoredData>) {
        Object.assign(this, props)
    }

    @PrimaryColumn_()
    id!: string

    @Index_()
    @ManyToOne_(() => Account, {nullable: true})
    depositor!: Account

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    actualDeposit!: bigint

    @Column_("numeric", {transformer: marshal.bigintTransformer, nullable: false})
    surplusDeposit!: bigint

    @Column_("bytea", {nullable: false})
    data!: Uint8Array
}
