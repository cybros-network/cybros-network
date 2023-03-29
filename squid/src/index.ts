import {TypeormDatabase} from "@subsquid/typeorm-store"
import {In} from "typeorm"
import {processor} from "./processor"

import {Account, Transfer} from "./model"
import {getAccount} from "./processor_helpers/account"
import {getBalanceTransfers} from "./processor_helpers/balance_transfers"

processor.run(new TypeormDatabase(), async ctx => {
    let transfersData = getBalanceTransfers(ctx)

    let accountIds = new Set<string>()
    for (let t of transfersData) {
        accountIds.add(t.from)
        accountIds.add(t.to)
    }

    let accounts = await ctx.store.findBy(Account, {id: In([...accountIds])}).then(accounts => {
        return new Map(accounts.map(a => [a.id, a]))
    })

    let transfers: Transfer[] = []

    for (let t of transfersData) {
        let {id, blockNumber, timestamp, extrinsicHash, amount, fee} = t

        let from = getAccount(accounts, t.from)
        let to = getAccount(accounts, t.to)

        transfers.push(new Transfer({
            id,
            blockNumber,
            timestamp,
            extrinsicHash,
            from,
            to,
            amount,
            fee
        }))
    }

    await ctx.store.save(Array.from(accounts.values()))
    await ctx.store.insert(transfers)
})
