use std::io::{self, Write};
use std::{thread::sleep, time::Duration};
use colored::Colorize;
use rand::Rng;

fn main() {
    println!("{}", "SUPER MEGA ULTRA OPSEC TOOL".green().bold());
    println!("{}", "Type help for help i guess".green());    

    loop {
        print!("{}", "┌──(root@kali)-[~]\n".red());
        print!("{}", "└──╼ ".red());
        io::stdout().flush().unwrap();

        let mut lol = String::new();
    
        io::stdin().read_line(&mut lol).expect("bul");

        let hui = lol.trim();

        if hui == "exit" {
             break;
        } else if hui == "help" {
             help();
        } else if hui == "proton vpn" {
             protonvpn();
             continue;
        } else if hui == "opera gx" {
             operagx();
             continue;
        } else if hui == "osint" {
             osint();
             continue;
        } else if hui == "sudo opsec" {
             sudoopsec();
             continue;
        } else {
             println!("{}", "wtf is this command".red());
        }
    }
}

fn help() {
    println!("{}", "
    exit       - Exit i guess,
    proton vpn - For install Proton VPN,
    opera gx   - ↑↑↑↑↑↑↑↑↑↑↑ Opera GX,
    osint      - ↑↑↑↑↑↑↑↑↑↑↑ Osint,
    sudo opsec - For opsec level: INFINITY,
    ".green());
}

fn protonvpn() {
    let servers = vec!["Germany", "Poland", "Antarctica", "Mars", "idfk"];
    let rand_index = rand::thread_rng().gen_range(0..servers.len());
    let rand_servers = servers[rand_index];
    println!("{}", "Install Proton VPN...".green());
    sleep(Duration::from_secs(1));
    println!("{}", "SUCCESFULY INSTALLED PROTON VPN".green().bold());
    sleep(Duration::from_millis(300));
    println!("{}", "Connecting to the server...".green());
    sleep(Duration::from_secs(2));
    println!("{}", format!("CONNECTED TO THE SERVER IN: {rand_servers}\n").green().bold());
}

fn operagx() {
    println!("{}", "Install Opera GX...".green());
    sleep(Duration::from_secs(1));
    println!("{}", "SUCCESSFULY INSTALLED OPERA GX HOLY OPSEC\n".green().bold());
}

fn osint() {
    println!("{}", "Installing osint...".green());
    sleep(Duration::from_secs(2));
    println!("{}", "SUCCESFULLY INSTALLED OSINT".green().bold());
    sleep(Duration::from_millis(300));
    println!("{}", "Mixing opsec and osint...".green());
    sleep(Duration::from_secs(1));
    println!("{}", "DONE\n".green().bold());
}

fn sudoopsec() {
    loop {
        let words = vec!["HOLY OPSEC", "OPSEC LEVEL: INFINITY", "MULLVAD VPN IS A BLOATWARE", "WINDOWS IS A BLOATWARE", "I USE KALI BTW"];
        let index = rand::thread_rng().gen_range(0..words.len());
        let rand_words = words[index];

        println!("{}", format!("{rand_words}").green().bold());
    }
}
