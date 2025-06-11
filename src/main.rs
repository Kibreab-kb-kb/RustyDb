use kbdb::*;
use std::io::{self,Write};




pub fn print_prompt(){
    print!("db >");
    io::stdout().flush().unwrap();
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