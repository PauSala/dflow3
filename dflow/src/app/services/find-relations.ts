import { DataModel, Table } from "../model/data-model";

/**
 * Gets all related tables
 * @param table 
 * @param model 
 * @returns 
 */
export const findRelations = (table: Table, model: DataModel) => {
    let tables: Record<string, Table> = {};
    let relations = [...table.relations];
    let visited: Set<number> = new Set();

    while (relations.length > 0) {
        let index = relations.pop() as number;
        visited.add(index);
        let relation = model.relations[`${index}`];
        let t1 = model.tables[relation.pk_table];
        let t2 = model.tables[relation.fk_table];
        let tRelations = [...t1.relations, ...t2.relations];
        for (const r of tRelations) {
            if (!visited.has(r)) {
                relations.push(r);
                visited.add(index);
            }
        }
    }

    for (const r of Array.from(visited)) {
        let relation = model.relations[r];
        let t1 = model.tables[relation.pk_table];
        let t2 = model.tables[relation.fk_table];
        if (t1.table_id !== table.table_id){
            tables[relation.pk_table] = t1;
        }
        if (t2.table_id !== table.table_id){
            tables[relation.fk_table] = t2;
        }
    }
    return tables;
}
