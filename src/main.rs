use std::io::{self,Write};

struct InputBuffer{
buffer:String,
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



fn main(){

    let mut input_buffer=InputBuffer::new();

    loop{
        print_prompt();
        read_input(&mut input_buffer);

        if input_buffer.buffer==".exit"{
            break;
        }
        else{
            println!("Unrecognized command '{}'.",input_buffer.buffer);
        }
    }

 }
