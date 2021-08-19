use super::replace_placeholder::replace_placeholder;

pub fn construct_select(table: Option<String>, items: Option<Vec<String>>) {
    let mut query = String::from("select $1 from $2 as $3;");

    if table.is_some() {
        replace_placeholder(&mut query, "$2", table.unwrap().as_str());
    }
}
