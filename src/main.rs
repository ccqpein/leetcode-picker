use clap::Clap;
use leetcode_picker::*;
use question::{Answer, Question};

fn print_code_snippet(arg: &Option<String>, qq: &Quiz) -> Result<(), String> {
    if let Some(ll) = arg {
        match qq.code_snippet(ll) {
            Some(cs) => println!("Code Snippet:\n{}", cs),
            None => return Err(format!("Cannot found {} code snippet", ll)),
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let commandline_args = cli_args::Args::parse();
    //dbg!(&commandline_args);

    // set token with command line token
    set_token(commandline_args.token());

    match commandline_args.if_random() {
        true => {
            let qq = if commandline_args.if_interact() {
                loop {
                    let qq = Quiz::get_randomly(commandline_args.level())?;
                    println!(
                        "{}",
                        qq.use_fmt_temp(
                            commandline_args.template(),
                            commandline_args.if_show_code_snippet()
                        )?
                    );

                    // ask
                    let a = Question::new("Is this good? (yes/no/y/n)")
                        .yes_no()
                        .until_acceptable()
                        .ask()
                        .unwrap();

                    if Answer::YES == a {
                        break qq;
                    }
                }
            } else {
                let qq = Quiz::get_randomly(commandline_args.level())?;
                println!(
                    "{}",
                    qq.use_fmt_temp(
                        commandline_args.template(),
                        commandline_args.if_show_code_snippet()
                    )?
                );
                qq
            };

            // show code snippet
            print_code_snippet(commandline_args.if_show_code_snippet(), &qq)?;
        }
        false => {
            // try id first
            if let Some(ref id) = commandline_args.quiz_id() {
                let qq = Quiz::get_by_id(*id)?;
                println!(
                    "{}",
                    qq.use_fmt_temp(
                        commandline_args.template(),
                        commandline_args.if_show_code_snippet()
                    )?
                );
                // show code snippet
                print_code_snippet(commandline_args.if_show_code_snippet(), &qq)?;
                return Ok(());
            }

            // try name then
            if let Some(ref name) = commandline_args.name() {
                let qq = Quiz::get_by_name(name)?;
                println!(
                    "{}",
                    qq.use_fmt_temp(
                        commandline_args.template(),
                        commandline_args.if_show_code_snippet()
                    )?
                );
                // show code snippet
                print_code_snippet(commandline_args.if_show_code_snippet(), &qq)?;
                return Ok(());
            }

            println!("If it is not random, need more info. Check -h")
        }
    }

    Ok(())
}
