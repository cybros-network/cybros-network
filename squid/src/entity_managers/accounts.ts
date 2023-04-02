import { Account } from "../model"
import { EntitiesManager } from "../entity_manager";

export default class AccountsManager extends EntitiesManager<Account> {
    constructor(entityClass?: typeof Account) {
        super({
            entityClass: entityClass ? entityClass : Account,
            newEntityFunc: id => new Account({id})
        });
    }
}