use std::io::{self,Write};

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
}

enum PrepareResult{
    Success,
    UnrecognizedStatement,
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
    if input_buffer.buffer.starts_with("insert"){
        statement.stmt_type=StatementType::Insert;
        PrepareResult::Success
    }else if input_buffer.buffer=="select"{
        statement.stmt_type=StatementType::Select;
        PrepareResult::Success
    }else{
        PrepareResult::UnrecognizedStatement
    }
}

fn execute_statement(statement:& Statement){
    match statement.stmt_type {
        StatementType::Insert=>{
            println!("This is where we would do an insert.");
        }

        StatementType::Select=>{
            println!("This is where we would do a select.");
        }
        
    }
}


fn main(){

    let mut input_buffer=InputBuffer::new();

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
        };

        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::Success=>(),
            PrepareResult::UnrecognizedStatement=>{
                println!("Unrecognized keyword at start of '{}'.",input_buffer.buffer);
                continue;
            }
        }
        execute_statement(&statement);
    }

 }