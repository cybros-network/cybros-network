import { EntitiesManager } from "../entity_manager";
import { Worker } from "../model"

export default class WorkersManager extends EntitiesManager<Worker> {
    constructor(entityClass?: typeof Worker) {
        super({
            entityClass: entityClass ? entityClass : Worker,
            newEntityFunc: id => new Worker({id})
        });
    }
}
