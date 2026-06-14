use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

//create helper function to determine if one or more conditions happen

pub fn row_matches (row: &Row, condition: &Condition, dataset: &Dataset) -> bool {
    match condition {
        //1. equal condition
        Condition::Equal(col_name, expected) => {
            let index = dataset.column_index (col_name);
            row.get_value(index) == expected
        }
        Condition::Not(inner) => {
            !row_matches(row, inner, dataset)
        }
        Condition::And(left, right) => {
            row_matches(row, left, dataset) && row_matches(row, right, dataset)
        }
        Condition::Or(left, right)=> {
            row_matches(row, left, dataset) || row_matches (row, right, dataset)
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut result = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
        if row_matches(row, filter, dataset) {
            result.add_row(row.clone());
        }
    }
    result
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}