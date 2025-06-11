use std::io::{self,Write};

#[derive(Debug)]
struct Row{
    id:u32,
    username:String,
    email:String,
}

struct  Table{ 
    rows:Vec<Row>,
    max_rows:usize,
}

impl Table{
    fn new()->Self{
        Table { 
        rows: Vec::new(),
        max_rows: 1400 
        }
    }
}


struct InputBuffer{
buffer:String,
}

enum MetaCommandResult{
    Success,
    UnrecognizedCommand,
}

enum StatementType{
    Insert,
    Select,
}

struct Statement{
stmt_type:StatementType,
row_to_insert:Option<Row>
}

enum PrepareResult{
    Success,
    UnrecognizedStatement,
    SyntaxError,
}



impl InputBuffer{
    fn new()->InputBuffer{
        InputBuffer { buffer: String::new(), }
    }
} 

fn print_prompt(){
    print!("db >");
    io::stdout().flush().unwrap();
}


fn read_input(input_buffer:&mut InputBuffer){
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

fn do_meta_command(input_buffer:&InputBuffer)->MetaCommandResult{
    match input_buffer.buffer.as_str() {
        ".exit"=>{
            std::process::exit(0);
        }
        _=>MetaCommandResult::UnrecognizedCommand
    }
}

fn prepare_statement(input_buffer:&InputBuffer,statement:&mut Statement)->PrepareResult{
    let parts:Vec<&str>=input_buffer.buffer.split_whitespace().collect();

    match parts.get(0) {
        
        Some(&"insert")=>{
            if parts.len()<4{
                return PrepareResult::SyntaxError;
            }
            let id=match parts[1].parse::<u32>(){
                Ok(n)=>n,
                Err(_)=>return PrepareResult::SyntaxError,
            };

            let username=parts[2].to_string();
            let email=parts[3].to_string();

            statement.stmt_type=StatementType::Insert;
            statement.row_to_insert=Some(Row{id,username,email});
            PrepareResult::Success
        }

        Some(&"select")=>{
            statement.stmt_type=StatementType::Select;
            statement.row_to_insert=None;
            PrepareResult::Success
        }

        _=>PrepareResult::UnrecognizedStatement,
    }
  
}

fn execute_statement(statement: &Statement, table: &mut Table) -> ExecuteResult {
    match statement.stmt_type {
        StatementType::Insert => execute_insert(statement, table),
        StatementType::Select => execute_select(table),
    }
}


enum ExecuteResult{
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


fn main() -> ! {

    let mut input_buffer=InputBuffer::new();
    let mut table=Table::new();


    loop{
        print_prompt();
        read_input(&mut input_buffer);

        if input_buffer.buffer.starts_with('.'){
            match do_meta_command(&input_buffer){
                MetaCommandResult::Success=>continue,
                MetaCommandResult::UnrecognizedCommand=>{
                    println!("Unrecognized command '{}'",input_buffer.buffer);
                    continue;
                }
            }
        }
        let mut statement=Statement{
            stmt_type:StatementType::Insert,
            row_to_insert:None,

        };

        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::Success=>(),
            PrepareResult::SyntaxError=>{
                println!("Syntax error. Could not parse statement.");
                continue;
            }
            PrepareResult::UnrecognizedStatement=>{
                println!("Unrecognized keyword at start of '{}'.",input_buffer.buffer);
                continue;
            }
        }
        match execute_statement(&statement, &mut table) {
            ExecuteResult::Success => println!("Executed."),
            ExecuteResult::TableFull => println!("Error: Table full."),
            ExecuteResult::Fail => println!("Execution failed."),
        }
    }

 }