use std::io::{self, stdin, stdout, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    loop {
        buffer.clear();
        print!("db > ");
        stdout().flush().unwrap();

        stdin().read_line(&mut buffer).expect("Failed to read line");

        let command = buffer.trim();
        if command.starts_with(".") {
            let meta_command_result = do_meta_command(command);
            match meta_command_result {
                META_COMMAND_SUCCESS => break,
                META_COMMAND_UNRECOGNIZED_COMMAND => println!("Unrecognized command '{}'", command),
            }
        }

        let error_str = format!("Unrecognized keyword at start of '{}'", command);
        let statement = prepare_statement(command).expect(&*error_str);
        execute_statement(statement);
    }
    Ok(())
}

enum MetaCommandResult {
    META_COMMAND_SUCCESS,
    META_COMMAND_UNRECOGNIZED_COMMAND,
}

enum PrepareStatementResult {
    PREPARE_SUCCESS,
    PREPARE_UNRECOGNIZED_STATEMENT,
}

enum StatementType {
    STATEMENT_INSERT,
    STATEMENT_SELECT,
}

fn do_meta_command(command: &str) -> MetaCommandResult {
    if command.eq(".exit") {
        return MetaCommandResult::META_COMMAND_SUCCESS;
    } else {
        return MetaCommandResult::META_COMMAND_UNRECOGNIZED_COMMAND;
    }
}

fn prepare_statement(command: &str) -> Result<StatementType, &str> {
    if command.starts_with("insert") {
        return Ok(StatementType::STATEMENT_INSERT);
    }
    if command.starts_with("select") {
        return Ok(StatementType::STATEMENT_SELECT);
    }

    return Err("Unrecognized keyword");
}

fn execute_statement(statement: StatementType) {
    match (statement) {
        StatementType::STATEMENT_INSERT => println!("This is where we would do an insert"),
        StatementType::STATEMENT_SELECT => println!("This is where we would do a select"),
    }
}
