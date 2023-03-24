use libmacchina::{GeneralReadout, MemoryReadout, BatteryReadout};
use colored::*;

fn main() {
    // ascii art
        let _linux = ["    .--.",
                     "   |o_o |",
                     "   |:_/ |",
                     "  //   \\ \\",
                     " (|     | )",
                     "/'\\_   _/`\\",
                     "\\___)=(___/"];
        let _macos = ["             ",
                     "          .:'",
                     "      __ :'__",
                     "   .'`__`-'__``.",
                     "  :__________.-'",
                     "  :_________:",
                     "   :_________`-;",
                     "    `.__.-.__.'"];
        let _windows = ["          _.-;;-._",
                       "   '-..-'|   ||   |",
                       "   '-..-'|_.-;;-._|",
                       "   '-..-'|   ||   |",
                       "   '-..-'|_.-''-._|"];


    // general readout
        use libmacchina::traits::GeneralReadout as _;
        let general_readout = GeneralReadout::new();

    // memory readout
        use libmacchina::traits::MemoryReadout as _;
        let memory_readout = MemoryReadout::new();
    
    // battery readout
        use libmacchina::traits::BatteryReadout as _;
        let battery_readout = BatteryReadout::new();

    // Uptime Related variables
        let uptime = general_readout.uptime().unwrap() as usize;
        let uptime_hours: String = ((uptime/60)/60).to_string();
        let uptime_minutes: String = ((uptime / 60) % 60).to_string();
        let uptime = format!("{} hours, {} minutes", uptime_hours, uptime_minutes);

    // General Info Variables
        let username = general_readout.username().unwrap();
        let os = general_readout.os_name().unwrap();
        let cpu = general_readout.cpu_model_name().unwrap();
        let total_ram = memory_readout.total().unwrap() as usize;
        let total_ram = total_ram / 1024;
        let used_ram = memory_readout.used().unwrap() as usize;
        let used_ram = used_ram / 1024;
        let machine = general_readout.machine().unwrap();
        let battery_percentage = battery_readout.percentage();
        let mut _battery_text: String = "".to_string();
    
    // Check if battery data is available
        match battery_percentage {
            Ok(res) => {_battery_text = res.to_string();},
            Err(_) => {_battery_text = "not available".to_string()}
        }
    
    // Determine Operating System and Print ASCII
    match os.to_lowercase().as_str() {
        "microsoft" => {
            for i in 0.._windows.len() {
                println!("    {}", _windows[i]);
            }
        },
        "macos" => {
            for i in 0.._macos.len() {
                println!("    {}", _macos[i]);
            }
        },
        _ => {
            for i in 0.._linux.len() {
                println!("    {}", _linux[i]);
            }
        },
    }

    println!("
    {}

    {}  {}
    {}    {}
    {}    {}
    {}   {} / {} MB
    {}   {}%

    {}

    {}   {}
    {} {}
    ", "~ system info ~".bright_blue(),
       "user".bright_yellow(), username,
       "os".bright_purple(), os,
       "up".bright_red(), uptime,
       "ram".bright_yellow(), used_ram, total_ram,
       "bat".bright_green(), _battery_text,
       "~ hardware info ~".bright_blue(),
       "cpu".bright_green(), cpu,
       "model".bright_cyan(), machine,
    );
}
