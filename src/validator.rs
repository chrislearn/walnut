use crate::Error;

pub fn is_valid(sql_type: &str, value: &str) -> Result<bool, Error> {
    match sql_type {
        "smallint" =>     
        "integer" =>      
        "int" =>          
        "bigint" =>       
        "numeric" =>      
        "decimal" =>      
        "money" =>        
        "money4" =>       
        "float" =>        
        "double" =>       
        "real" =>         
        "date" =>         
        "time" =>         
        "timestamp" =>    
        "bit" =>          
        "char" =>         
        "varchar" =>      
        "nvarchar" =>     
        "longvarchar" =>  
        "longnvarchar" => 
        "refcursor" =>    
        "blob" =>         
        "clob" =>         
        "idstamp" =>      
        "tinyint" =>      
        "boolean" =>
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
fn is_money4(value: &str) -> bool {
    
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
fn is_datetimeoffset(value: &str) -> bool {
    
}
fn is_time(value: &str) -> bool {
    
}
fn is_timeoffset(value: &str) -> bool {
    
}