import { EntitiesManager } from "./entity_manager";
import {
    Account,
    Impl, ImplBuild,
    Worker,
    Pool, CreatingTaskPolicy, PoolWorkers,
    Task,
} from "./model"

export class AccountsManager extends EntitiesManager<Account> {
    constructor(entityClass?: typeof Account) {
        super({
            entityClass: entityClass ? entityClass : Account,
            newEntityFunc: id => new Account({id})
        });
    }
}

export class ImplsManager extends EntitiesManager<Impl> {
    constructor(entityClass?: typeof Impl) {
        super({
            entityClass: entityClass ? entityClass : Impl,
            newEntityFunc: id => new Impl({id})
        });
    }
}

export class ImplBuildsManager extends EntitiesManager<ImplBuild> {
    constructor(entityClass?: typeof ImplBuild) {
        super({
            entityClass: entityClass ? entityClass : ImplBuild,
            newEntityFunc: id => new ImplBuild({id})
        });
    }
}

export class WorkersManager extends EntitiesManager<Worker> {
    constructor(entityClass?: typeof Worker) {
        super({
            entityClass: entityClass ? entityClass : Worker,
            newEntityFunc: id => new Worker({id})
        });
    }
}

export class PoolsManager extends EntitiesManager<Pool> {
    constructor(entityClass?: typeof Pool) {
        super({
            entityClass: entityClass ? entityClass : Pool,
            newEntityFunc: id => new Pool({id})
        });
    }
}

export class CreatingTaskPoliciesManager extends EntitiesManager<CreatingTaskPolicy> {
    constructor(entityClass?: typeof CreatingTaskPolicy) {
        super({
            entityClass: entityClass ? entityClass : CreatingTaskPolicy,
            newEntityFunc: id => new CreatingTaskPolicy({id})
        });
    }
}

export class PoolWorkersManager extends EntitiesManager<PoolWorkers> {
    constructor(entityClass?: typeof PoolWorkers) {
        super({
            entityClass: entityClass ? entityClass : PoolWorkers,
            newEntityFunc: id => new PoolWorkers({id})
        });
    }
}

export class TasksManager extends EntitiesManager<Task> {
    constructor(entityClass?: typeof Task) {
        super({
            entityClass: entityClass ? entityClass : Task,
            newEntityFunc: id => new Task({id})
        });
    }
}
