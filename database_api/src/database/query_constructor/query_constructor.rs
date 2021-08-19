use super::replace_placeholder::replace_placeholder;

const ITEMS_PLACEHOLDER: &str = "$items";
const TABLE_PLACEHOLDER: &str = "$table_placeholder";
const TABLE_NAME_PLACEHOLDER: &str = "$table_name_placeholder";
const WHERE_PLACEHOLDER: &str = "$where_placeholder";

#[derive(PartialEq)]
pub enum DbOperation {
    Select,
    SelectDistinct,
    Insert,
    Update,
    Delete,
    Unknown,
}

///Create an SQL query based on operation, items, where conditions and so on.
///
///Example: SELECT {items} FROM {table} {t} WHERE {where_condition};
pub fn construct_operation(
    db_operation: DbOperation,
    table: Option<String>,
    items: Option<Vec<String>>,
    where_condition: Option<String>,
) -> String {
    if items.is_none() && table.is_none() || db_operation == DbOperation::Unknown {
        return "".to_string();
    }

    let mut query = construct_query(&db_operation);

    //Populate query with items
    populate_query_with_items(&mut query, &items.unwrap());

    //Populate query with table
    populate_query_with_table(&mut query, &table.unwrap());

    //Populate query with where condition, if there is such given
    if db_operation == DbOperation::Update || where_condition.is_some() {
        populate_query_with_where(&mut query, &where_condition.unwrap())
    }

    return query;
}

fn populate_query_with_items(query: &mut String, items: &Vec<String>) {
    let mut concat_str = String::new();
    for i in 0..items.len() - 1 {
        concat_str += &items[i];

        if i < items.len() - 1 {
            concat_str += ", ";
        }
    }

    replace_placeholder(query, ITEMS_PLACEHOLDER, &concat_str);
}

fn populate_query_with_table(query: &mut String, table_name: &str) {
    replace_placeholder(query, TABLE_PLACEHOLDER, &table_name);
    replace_placeholder(query, TABLE_NAME_PLACEHOLDER, &table_name[0..0]);
}

fn populate_query_with_where(query: &mut String, where_condition: &str) {
    replace_placeholder(query, WHERE_PLACEHOLDER, where_condition);
}

fn construct_query(db_opeartion: &DbOperation) -> String {
    match db_opeartion {
        DbOperation::Select => {
            return format!(
                "select {0} from {1} {2};",
                ITEMS_PLACEHOLDER, TABLE_PLACEHOLDER, TABLE_NAME_PLACEHOLDER
            );
        }
        DbOperation::SelectDistinct => {
            return format!(
                "select {0} from {1} {2};",
                ITEMS_PLACEHOLDER, TABLE_PLACEHOLDER, TABLE_NAME_PLACEHOLDER
            );
        }
        DbOperation::Insert => {
            return format!(
                "insert into {1} {2} values ({0});",
                ITEMS_PLACEHOLDER, TABLE_PLACEHOLDER, TABLE_NAME_PLACEHOLDER
            );
        }
        DbOperation::Update => {
            return format!(
                "update {1} {2} set {0} where {3};",
                ITEMS_PLACEHOLDER, TABLE_PLACEHOLDER, TABLE_NAME_PLACEHOLDER, WHERE_PLACEHOLDER
            );
        }
        DbOperation::Delete => {
            return format!(
                "delete from {0} {1} where {2};",
                TABLE_PLACEHOLDER, TABLE_NAME_PLACEHOLDER, WHERE_PLACEHOLDER
            );
        }
        DbOperation::Unknown => return "".to_string(),
    };
}
