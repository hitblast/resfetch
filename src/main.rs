/* MIT License

Copyright (c) 2024-present HitBlast
Copyright (c) 2022-2024 furtidev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */


// imports
use libmacchina::{traits::BatteryState, BatteryReadout, GeneralReadout, MemoryReadout};
use colored::*;

fn main() {
    // ascii art
    let _linux = [
        "    .--.",
        "   |^_^ |",
        "   |:_/ |",
        "  //   \\ \\",
        " (|     | )",
        "/'\\_   _/`\\",
        "\\___)=(___/"
    ];
    let _linux_huh = [
        "    .--.",
        "   |o_o |  ??",
        "   |:_/ |",
        "  //   \\ \\",
        " (|     | )",
        "/'\\_   _/`\\",
        "\\___)=(___/"
    ];
    let _macos = [
        "             ",
        "          .:'",
        "      __ :'__",
        "   .'`__`-'__``.",
        "  :__________.-'",
        "  :_________:",
        "   :_________`-;",
        "    `.__.-.__.'"
    ];
    let _windows = [
        "          _.-;;-._",
        "   '-..-'|   ||   |",
        "   '-..-'|_.-;;-._|",
        "   '-..-'|   ||   |",
        "   '-..-'|_.-''-._|"
    ];

    // general readout
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();

    // memory readout
    use libmacchina::traits::MemoryReadout as _;
    let memory_readout = MemoryReadout::new();
    
    // battery readout
    use libmacchina::traits::BatteryReadout as _;
    let battery_readout = BatteryReadout::new();

    // uptime-related variables
    let uptime: usize = general_readout.uptime().unwrap_or_default();
    let uptime_days: String = (uptime / (60 * 60 * 24)).to_string();
    let uptime_hours: String = ((uptime / 60) / 60).to_string();
    let uptime_minutes: String = ((uptime / 60) % 60).to_string();
    let mut uptime: String = format!("{} minutes", uptime_minutes);

    if uptime_hours != "0" {
        uptime = format!("{} hours, {}", uptime_hours, uptime);
    }
    if uptime_days != "0" {
        uptime = format!("{} days, {}", uptime_days, uptime);
    }

    // information variables
    let username = general_readout.username().unwrap_or("unknown".to_string());
    let os = general_readout.os_name().unwrap_or("unknown".to_string());
    let distro = general_readout.distribution().unwrap_or_default();
    let cpu = general_readout.cpu_model_name().unwrap_or_default();
    let cpu_cores = general_readout.cpu_cores().unwrap_or_default();
    let total_ram = memory_readout.total().unwrap_or_default() / 1024;
    let used_ram = memory_readout.used().unwrap_or_default() / 1024;
    let machine = general_readout.machine().unwrap_or_default();

    // battery information
    // (displayed only if battery is detected )
    let battery_percentage = battery_readout.percentage();
    let mut _battery_text: String = "".to_string();
    
    match battery_percentage {
        Ok(res) => {
            _battery_text = res.to_string() + "%";

            match battery_readout.status() {
                Ok(status) => {
                    match status {
                        BatteryState::Charging => {_battery_text = _battery_text + " (charging)"},
                        BatteryState::Discharging => {_battery_text = _battery_text + " (discharging)"}
                    }
                },
                Err(_) => {}
            }
        },
        Err(_) => {_battery_text = "not found".to_string()}
    }

    // clear terminal window before displaying ASCII
    print!("\x1B[2J\x1B[1;1H");
    
    // determine operating system and print ASCII
    match os.to_lowercase().split_whitespace().next() {
        Some(platform) => {
            match platform {
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
        },
        None => {
            for i in 0.._linux_huh.len() {
                println!("    {}", _linux_huh[i]);
            }
        }
    }

    // print system info
    println!("
    {}

    {}  {}
    {}    {} [{}]
    {}    {}
    {}   {} / {} MB
    {}   {}
    ", "~ system info ~".bright_blue(),
    "user".bright_yellow(), username,
    "os".bright_purple(), os, distro.bright_yellow(),
    "up".bright_red(), uptime,
    "ram".bright_yellow(), used_ram, total_ram,
    "bat".bright_green(), _battery_text,
    );

    // print cpu info
    if !cpu.is_empty() || !machine.is_empty() {
    println!("    {}

    {} {} ({} cores)
    {} {}
    ",
    "~ hardware info ~".bright_blue(),
    "cpu".bright_green(), cpu, cpu_cores,
    "model".bright_cyan(), machine,
    );
    }
}
