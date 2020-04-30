use crate::Error;

pub fn is_valid(sql_type: &str, value: &str) -> Result<bool, Error> {
    match sql_type {
        "smallint" =>     is_smallint,
        "integer" =>      is_integer,
        "int" =>          is_integer,
        "bigint" =>       is_bigint,
        "numeric" =>      is_numeric,
        "decimal" =>      is_numeric.
        "money" =>        is_money,
        "float" =>        is_numeric,
        "double" =>       is_numeric,
        "real" =>         is_numeric,
        "date" =>         is_date,
        "time" =>         is_time,
        "timestamp" =>    is_datetime,
        "bit" =>          is_bit,
        "char" =>         is_string,
        "varchar" =>      is_string,
        "nvarchar" =>     is_string,
        "longvarchar" =>  is_string,
        "longnvarchar" => is_string,  
        "boolean" =>    is_bool,
        _ => {return Err(Error::General("type is invalid or not support".into()))},
    }
}


fn is_string(value: &str) -> bool {

}

fn is_numeric(value: &str) -> bool {

}
fn is_smallint(value: &str) -> bool {
    
}
fn is_integer(value: &str) -> bool {
    
}
fn is_bigint(value: &str) -> bool {
    
}
fn is_money(value: &str) -> bool {
    
}
fn is_bit(value: &str) -> bool {
    
}
fn is_boolean(value: &str) -> bool {
    
}
fn is_date(value: &str) -> bool {
    
}
fn is_datetime(value: &str) -> bool {
    
}
fn is_time(value: &str) -> bool {
    
}