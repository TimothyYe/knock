use std::process::Command;

pub fn execute_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut parts = command.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    Command::new(command).args(args).spawn()?.wait()?;

    Ok(())
}

mod tests {
    #[test]
    fn test_execute_command() {
        let result = crate::executor::execute_command("ls -lh ./");
        assert!(result.is_ok());
    }
}
