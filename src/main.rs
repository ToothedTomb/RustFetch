use sysinfo::{System, SystemExt, ProcessorExt};
use termion::{color, style};
use nvml_wrapper::Nvml;
use std::process::Command;

fn main() {
    // Create a System object to fetch system information
    let mut sys = System::new_all();
    sys.refresh_all();

    // Fetching information
    let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());
    let os_name = sys.name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = sys.os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = sys.kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let cpu_brand = sys.processors().get(0).map(|p| p.brand().to_string()).unwrap_or_else(|| "Unknown".to_string());
    let cpu_cores = sys.processors().len();
    
    // Fetch uptime
    let uptime = sys.uptime();
    let hours = uptime / 36000;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;
    // Initialize NVML and fetch GPU information
    let nvml = Nvml::init().unwrap();
    let device_count = nvml.device_count().unwrap();
    let mut gpu_info = Vec::new();

    for i in 0..device_count {
        let device = nvml.device_by_index(i).unwrap();
        let name = device.name().unwrap();
        let memory = device.memory_info().unwrap();
        gpu_info.push((name, memory.total, memory.used));
    }

    // Fetch desktop environment
    let desktop_env = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string());

    // Fetch shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());

    let tux = r#"                    
                    ..- - .              
                   '        `.           
                  '.- .  .--. .          
                 |: _ | :  _ :|          
                 |`(@)--`.(@) |          
                 : .'     `-, :          
                 :(_____.-'.' `          
                 : `-.__.-'   :          
                 `  _.    _.   .         
                /  /  `_ '  \    .       
               .  :          \\   \      
              .  : _      __  .\   .     
             .  /             : `.  \    
            :  /      '        : `.  .   
           '  `      :          : :  `.  
         .`_ :       :          / '   |  
         :' \ .      :           '__  :  
      .--'   \`-._    .      .' :    `).  
    ..|       \   )          :   '._.'  : 
   ;           \-'.        ..:         / 
   '.           \  - ....-   |        '  
      -.         :   _____   |      .'   
        ` -.    .'--       --`.   .'     
            `--                --        
  "#;

    // Displaying information
    println!("{}{}{}",
             color::Fg(color::Cyan), style::Bold, "Rustyfetch");
    println!("{}",tux);
    println!("{}Hostname:{} {}", color::Fg(color::Green), style::Reset, hostname);
    println!("{}OS:{} {} {}", color::Fg(color::Green), style::Reset, os_name, os_version);
    println!("{}Kernel:{} {}", color::Fg(color::Green), style::Reset, kernel_version);
    println!("{}Memory:{} {} / {} KB", color::Fg(color::Green), style::Reset, used_memory, total_memory);
    println!("{}CPU:{} {} ({} cores)", color::Fg(color::Green), style::Reset, cpu_brand, cpu_cores);
    println!("{}Uptime:{} {:02}:{:02}:{:02}", color::Fg(color::Green), style::Reset, hours, minutes, seconds);    println!("{}Desktop Environment:{} {}", color::Fg(color::Green), style::Reset, desktop_env);
    println!("{}Shell:{} {}", color::Fg(color::Green), style::Reset, shell);
    for (i, (name, total, used)) in gpu_info.iter().enumerate() {
        println!("{}GPU {}:{} {} ({} / {} bytes)", color::Fg(color::Green), i + 1, style::Reset, name, used, total);
    }
    println!("This program has been created by Jonathan Steadman");
}
