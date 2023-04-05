import { EntitiesManager } from "../entity_manager";
import { Task } from "../model"

export default class TasksManager extends EntitiesManager<Task> {
    constructor(entityClass?: typeof Task) {
        super({
            entityClass: entityClass ? entityClass : Task,
            newEntityFunc: id => new Task({id})
        });
    }
}
