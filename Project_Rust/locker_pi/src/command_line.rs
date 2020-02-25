
#[derive(Debug)]
pub enum Cmd {
    Help, List, Add, Remove, Modify, None,
}
/*
#[derive(Debug)]
pub enum SubCmdList {
    User, Restrictions, Type, Log,
}

#[derive(Debug)]
pub enum SubCmdAdd {
    Admin, Owner, Restricted,
}

#[derive(Debug)]
pub enum SubCmdModify {
    Admin, Owner, Restricted,
}
*/


pub fn parse_cmd(c: &String) -> Cmd {
    if c == "-help" { return Cmd::Help }
    if c == "-list" { return Cmd::List }
    if c == "-add" { return Cmd::Add }
    if c == "-remove" { return Cmd::Remove }
    if c == "-modify" { return Cmd::Modify }
    Cmd::None
}

pub fn handle_usage() {
    println!("\nUsage:\tlockerpi\n");

    println!("-list\t\t\t\tlists all users");
    println!("\t-u <username>\tlist specific user");
    println!("\t-r\t\tlist restrictions");
    println!("\t-t\t\t\tlist user type");
    println!("\t-l\t\t\tlist access log\n");

    println!("-add <username>\t\t\tadds a new user, returns a user access code");
    println!("\t-a\t\t\tset as admin: host device access");
    println!("\t-o\t\t\tset as owner: unrestricted lock use");
    println!("\t-r <options>\t\tset as restricted: restricted lock use");
    println!("\t\t\t\t(example options)");
    println!("\t\t\t\t\t_mon_|_wed_ (all day Mondays or Wednesdays)");
    println!("\t\t\t\t\t_mon_8:00am (Mondays before 8:01am)");
    println!("\t\t\t\t\t11/28/2019_ (anytime starting on the 28th)");
    println!("\t\t\t\t\tmon__tue|10x (range: Monday through Tuesday,");
    println!("\t\t\t\t\t\t\t10 lock/unlock cycles max)\n");

    println!("-remove <username>\t\tremoves a user\n");

    println!("-modify <username>\t\tmodifies current user");
    println!("\t[-a|-o|-r <options>]	see -add");
    println!("\t-n <new username>\tchanges username\n");

    println!("-help \t\t\tdisplays valid commands and usage\n");
}

pub fn handle_list(_cmd: Vec<String>) {
    
}

pub fn handle_add(_cmd: Vec<String>) {

}

pub fn handle_remove(_cmd: Vec<String>) {

}

pub fn handle_modify(_cmd: Vec<String> ){

}


/*
    pub mod time_window;

    use time_window as tm;
    ...

    let t = "02:50am";
    
    let tstr = tm::Time::new_validated(t.to_string());

    println!("Time = {}\n", t.to_string());
    match tstr {
        Ok(e) => println!("{}", e.to_string()),
        Err(e) => println!("{:?}", e),
    }
*/