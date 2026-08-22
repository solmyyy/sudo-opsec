use colored::Colorize;
use rand::Rng;
use std::io::{self, Write};
use std::{thread::sleep, time::Duration};

#[allow(non_upper_case_globals)]
const qwjfqpo: &str = include_str!("../opseclevel.txt");

fn main() {
    println!("{}", "SUPER MEGA ULTRA OPSEC TOOL".green().bold());
    sleep(Duration::from_millis(300));
    println!("{}", "Type help for help i guess".green());
    sleep(Duration::from_millis(200));

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
    println!(
        "{}",
        "
    exit       - Exit i guess,
    proton vpn - For install Proton VPN,
    opera gx   - ↑↑↑↑↑↑↑↑↑↑↑ Opera GX,
    osint      - ↑↑↑↑↑↑↑↑↑↑↑ Osint,
    sudo opsec - For opsec level: INFINITE
    "
        .green()
    );
}

fn protonvpn() {
    let servers = vec![
        "Germany",
        "Poland",
        "Antarctica",
        "Mars",
        "idfk",
        "Proxima Centauri",
        "Opsec",
    ];
    let rand_index = rand::thread_rng().gen_range(0..servers.len());
    let rand_servers = servers[rand_index];
    println!("{}", "Install Proton VPN...".green());
    sleep(Duration::from_secs(1));
    println!("{}", "SUCCESFULY INSTALLED PROTON VPN".green().bold());
    sleep(Duration::from_millis(300));
    println!("{}", "Connecting to the server...".green());
    sleep(Duration::from_secs(2));
    println!(
        "{}",
        format!("CONNECTED TO THE SERVER IN: {rand_servers}\n")
            .green()
            .bold()
    );
}

fn operagx() {
    println!("{}", "Install Opera GX...".green());
    sleep(Duration::from_secs(1));
    println!(
        "{}",
        "SUCCESSFULY INSTALLED OPERA GX HOLY OPSEC\n".green().bold()
    );
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
    for _ in 0..10000 {
        let words = vec![
            "HOLY OPSEC",
            "OPSEC LEVEL: INFINITE",
            "MULLVAD VPN ACTIVATED",
            "WINDOWS IS A BLOATWARE",
            "I USE KALI BTW",
            "MR. ROBOT",
            "FSOCIETY",
            "SUDO APT INSTALL OPSEC",
        ];

        let index = rand::thread_rng().gen_range(0..words.len());
        let rand_words = words[index];

        println!("{}", format!("{rand_words}").green().bold());
        sleep(Duration::from_millis(001));
    }

    matrix();

    println!("{}", format!("{qwjfqpo}").green());
}

fn matrix() {
    for _ in 0..5000 {
        let line: String = (0..1)
            .map(|_| if rand::random::<bool>() { '0' } else { '1' })
            .collect();
        println!("{}", line.green().bold());
        sleep(Duration::from_millis(50));
    }
}

// qwerty123
