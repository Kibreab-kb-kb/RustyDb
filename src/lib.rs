use std::io::{self,Write};
pub struct Row{
    pub id:u32,
    pub username:String,
    pub email:String,
}

pub struct  Table{ 
    pub rows:Vec<Row>,
    pub max_rows:usize,
}

impl Table{
    pub fn new()->Self{
        Table { 
        rows: Vec::new(),
        max_rows: 1400 
        }
    }
}


pub struct InputBuffer{
pub buffer:String,
}

pub enum MetaCommandResult{
    Success,
    UnrecognizedCommand,
}

pub enum StatementType{
    Insert,
    Select,
}

pub struct Statement{
pub stmt_type:StatementType,
pub row_to_insert:Option<Row>
}

pub enum PrepareResult{
    Success,
    UnrecognizedStatement,
    SyntaxError,
}



impl InputBuffer{
    pub fn new()->InputBuffer{
        InputBuffer { buffer: String::new(), }
    }
} 


 pub fn read_input(input_buffer:&mut InputBuffer){
    input_buffer.buffer.clear();  
    let bytes_read=io::stdin()
    .read_line(&mut input_buffer.buffer)
    .expect("Failed to read line");

    if bytes_read==0{
        println!("Error reading input");
        std::process::exit(1);

    }

    if input_buffer.buffer.ends_with('\n'){
        input_buffer.buffer.pop();
        if input_buffer.buffer.ends_with('\r'){
            input_buffer.buffer.pop();
        }
    }



}

pub fn do_meta_command(input_buffer:&InputBuffer)->MetaCommandResult{
    match input_buffer.buffer.as_str() {
        ".exit"=>{
            std::process::exit(0);
        }
        _=>MetaCommandResult::UnrecognizedCommand
    }
}

pub fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    let parts: Vec<&str> = input_buffer.buffer.split_whitespace().collect();

    match parts.get(0) {
        Some(&"insert") => {
            if parts.len() < 4 {
                return PrepareResult::SyntaxError;
            }

            let id = match parts[1].parse::<i32>() {
                Ok(n) if n >= 0 => n as u32,
                Ok(_) => {
                    println!("Error: ID must be a non-negative integer.");
                    return PrepareResult::SyntaxError;
                }
                Err(_) => {
                    println!("Error: ID must be a number.");
                    return PrepareResult::SyntaxError;
                }
            };
            

            let username = parts[2].to_string();
            let email = parts[3].to_string();

            if username.len() > 32 || email.len() > 255 {
                return PrepareResult::SyntaxError;
            }

            statement.stmt_type = StatementType::Insert;
            statement.row_to_insert = Some(Row { id, username, email });
            PrepareResult::Success
        }

        Some(&"select") => {
            statement.stmt_type = StatementType::Select;
            statement.row_to_insert = None;
            PrepareResult::Success
        }

        _ => PrepareResult::UnrecognizedStatement,
    }
}

pub fn execute_statement(statement: &Statement, table: &mut Table) -> ExecuteResult {
    match statement.stmt_type {
        StatementType::Insert => execute_insert(statement, table),
        StatementType::Select => execute_select(table),
    }
}


pub enum ExecuteResult{
    TableFull,
    Success,
    Fail
}

fn execute_insert(statement:& Statement,table:&mut Table)->ExecuteResult{
    if table.rows.len() >= table.max_rows {
        return ExecuteResult::TableFull;
    }

    if let Some(row)=&statement.row_to_insert{
        table.rows.push(Row 
            { id: row.id, 
              username:row.username.clone(), 
              email: row.email.clone() 
            });
            ExecuteResult::Success
    }else{
        ExecuteResult::Fail
    }
}
 fn execute_select(table: &Table) -> ExecuteResult {
    for row in &table.rows {
        println!("({}, {}, {})", row.id, row.username, row.email);
    }
    ExecuteResult::Success
}