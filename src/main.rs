use std::env;
use std::string::String;
use std::convert::AsRef;

enum BackupTypes {
    SQL,
    Mongo,
    Asterisk,
    ETC,
    Unknown
}

fn backuptype_to_str(back_type: &BackupTypes) -> &'static str {
    match *back_type {
        BackupTypes::SQL => "SQL",
        BackupTypes::Mongo => "Mongo",
        BackupTypes::Asterisk => "Asterisk",
        BackupTypes::ETC => "ETC",
        BackupTypes::Unknown => "Unknown"
    }
}

struct BackupInst {
    backup_type: BackupTypes,
    arguments: Vec<String>
}

impl BackupInst {
    fn new(params: &Vec<String>) -> BackupInst {
        let params_len = params.len();

        if params_len > 0 {
            let mut cur_backup = match params[0].as_ref() {
                "--databases" | "-dbs" => {
                    BackupInst {
                        backup_type: BackupTypes::SQL,
                        arguments: Vec::new()
                    }
                }
                "--etc" | "-etc" => {
                    BackupInst {
                        backup_type: BackupTypes::ETC,
                        arguments: Vec::new()
                    }
                }
                "--asterisk" | "-ask" => {
                    BackupInst {
                        backup_type: BackupTypes::Asterisk,
                        arguments: Vec::new()
                    }
                }
                "--mongodb" | "-mongo" => {
                    BackupInst {
                        backup_type: BackupTypes::Mongo,
                        arguments: Vec::new()
                    }
                }
                _ => BackupInst {
                    backup_type: BackupTypes::Unknown,
                    arguments: Vec::new()
                }
            };

            for i in 1..params_len {
                cur_backup.arguments.push(String::from(params[i].as_ref()));
            }

            cur_backup
        }
        else {
            BackupInst{
                backup_type: BackupTypes::Unknown,
                arguments: Vec::new()
            }
        }
    }

    fn params_dump(&self) -> () {
        println!("");
        println!("Type of backup Instance: {}", backuptype_to_str(&self.backup_type));
        println!("Arguments for Backup Instance: ");
        let args = &self.arguments;
        let args_num = args.len();
        for i in 0..args_num {
            println!("{}", args[i]);
        }

        println!("==============================================");
        println!("");
    }
}

fn parse_args() -> Vec<BackupInst> {
    let mut answ_vec:Vec<BackupInst> = Vec::new();
    let args: Vec<_> = env::args().collect();
    let mut groupped_args: Vec<Vec<String>> = Vec::new();

    for i in 1..args.len() {
        let cur_arg = args[i].to_string();
        let cur_pref = cur_arg.chars().next().unwrap();
        
        match cur_pref {
            '-' => {
                let mut new_vector: Vec<String> = Vec::new();
                new_vector.push(cur_arg);
                groupped_args.push(new_vector);
            }
            _ => {
                let cur_length = groupped_args.len();
                let mut cur_vector: &mut Vec<String> = &mut groupped_args[cur_length - 1];
                cur_vector.push(cur_arg);
            }
        }
    }

    let opts_len = groupped_args.len();

    for i in 0..opts_len {
        answ_vec.push(BackupInst::new(&groupped_args[i]));

        answ_vec[i].params_dump();
    }

    answ_vec
}

fn main() {

    parse_args();
}
