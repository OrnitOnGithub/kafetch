use std::fs;

fn main() {
  // TODO:
  // DONE - pretty_name
  // DONE - kernel name
  // DONE - hostname
  // DONE - username
  // DONE - computer name
  // DONE - shell
  // DONE - pakage manager name
  // DONE - CPU
  // DONE - GPU(s)
  // DONE - RAM
  // - resolution
  // - DE
  // - WM
  // - terminal
  // - display

  // parse os-release file to get PRETTY_NAME
  let mut pretty_name = "";

  let os_release_info = fs::read_to_string("/etc/os-release")
    .expect(format!("Could not read {}", "/etc/os-release").as_str());
  let os_release_info_lines = os_release_info.split('\n');
  // read all lines. find the name of the field by taking everything before the `=`
  // if the field is PRETTY_NAME, put everything after into pretty_name var.
  for line in os_release_info_lines {
    let index_of_equal = index_first_character(line.to_string(), '=');
    let field_name = line[0..index_of_equal].to_string();
    if field_name == "PRETTY_NAME" {
      pretty_name = &line[index_of_equal+2..line.len()-1]; //dont grab the first and last characters as well - they're quotation marks
    }
  }

  // get kernel name with `uname -r`
  let mut kernel_name = std::process::Command::new("uname");
  kernel_name.arg("-r");
  let mut kernel_name = String::from_utf8(kernel_name.output().expect("Failed to run `uname -r`.").stdout).expect("");
  kernel_name.pop(); // remove the newline at the end

  // get hostname by running `hostname`
  let mut hostname = std::process::Command::new("hostname");
  let mut hostname = String::from_utf8(hostname.output().expect("Failed to run `hostname`.").stdout).expect("");
  hostname.pop(); // remove the newline at the end

  // get computer vendor and model with `cat /sys/devices/virtual/dmi/id/sys_vendor`
  let mut vendor = std::process::Command::new("cat");
  vendor.arg("/sys/devices/virtual/dmi/id/sys_vendor");
  let mut vendor = String::from_utf8(vendor.output().expect("Failed to run `cat /sys/devices/virtual/dmi/id/sys_vendor`.").stdout).expect("");
  vendor.pop(); // remove the newline at the end
  // and `cat /sys/devices/virtual/dmi/id/product_name`
  let mut model = std::process::Command::new("cat");
  model.arg("/sys/devices/virtual/dmi/id/product_name");
  let mut model = String::from_utf8(model.output().expect("cat /sys/devices/virtual/dmi/id/product_name`.").stdout).expect("");
  model.pop(); // remove the newline at the end
  let vendor_and_model = format!("{} {}", vendor, model);

  // get username by running `whoami`
  let mut user_name = std::process::Command::new("whoami");
  let mut user_name = String::from_utf8(user_name.output().expect("Failed to run `whoami`.").stdout).expect("");
  user_name.pop(); // remove the newline at the end

  // get the value of the shell environment variable
  let shell_name = std::env::var("SHELL").expect("Failed to get value of SHELL environment variable.");

  // get the package manager -- try every possibility
  let mut package_manager = "cannot find";

  let package_managers: Vec<&str> = vec!["apt", "dnf", "pacman", "dpkg", "yum", "nix", "netpkg", "rpm"];

  for package_manager_attempt in package_managers {
    let mut command_attempt = std::process::Command::new(package_manager_attempt);
    let command_attempt = command_attempt.output();
    match command_attempt {
      Ok(_) => {
        package_manager = package_manager_attempt;
        break;
      },
      Err(_) => {
        continue;
      }
    }
  }

  // get uptime with `uptime -p`
  let mut uptime = std::process::Command::new("uptime");
  uptime.arg("-p");
  let mut uptime = String::from_utf8(uptime.output().expect("Failed to run `uptime .p`").stdout).expect("");
  uptime.pop(); // remove the newline at the end
  uptime.remove(0); uptime.remove(0); uptime.remove(0); // remove first three characters: "up "

  

  // parse `lscpu` to get cpu count, name

  let mut cpu_info = std::process::Command::new("lscpu");
  let mut cpu_info = String::from_utf8(cpu_info.output().expect("Failed to run `lscpu`").stdout).expect("");
  cpu_info.pop(); // remove the newline at the end

  let cpu_info_lines = cpu_info.split('\n');
  let mut cpu_count = "";
  let mut cpu_name: Vec<&str> = Vec::new();

  for line in cpu_info_lines {
    let index_of_colon = index_first_character(line.to_string(), ':');
    let field_name = line[0..index_of_colon].to_string();
    match field_name.as_str() {
      "CPU(s)" => {
        let info = line[index_of_colon+1..line.len()].split(' ');
        for element in info {
          cpu_count = element;
        }
      }
      "Model name" => {
        let info = line[index_of_colon+1..line.len()].split(' ');
        for element in info {
          cpu_name.push(element);
        }
      }
      _ => {} // ignore
    }
  }

  // remove all random blank strings
  let mut clean_cpu_name: Vec<&str> = Vec::new();
  for element in cpu_name {
    if element != "" {
      clean_cpu_name.push(element);
    }
  }

  let cpu_name = clean_cpu_name.join(" ");



  // run `lspci`
  let mut lspci = std::process::Command::new("lspci");
  let pci_info = String::from_utf8(lspci.output().expect("Failed to run `lspci`").stdout).expect("");

  let pci_info_lines = pci_info.split('\n');

  let mut vga_info: Vec<&str> = Vec::new();

  for line in pci_info_lines {
    let split_line = line.split_whitespace();
      for field in split_line {
        if field == "VGA" {
          let index_of_first_colon = index_first_character(line.to_string(), ':');
          let index_of_second_colon = index_first_character(line[index_of_first_colon+1..line.len()].to_string(), ':');
          vga_info.push(&line[index_of_second_colon+5..line.len()]);
        }
      }
  }

  // If no GPU detected
  if vga_info.len() == 0 {
    vga_info.push("No GPU");
  }

  // get memory information with `cat /proc/meminfo`
  let mut meminfo = std::process::Command::new("cat");
  meminfo.arg("/proc/meminfo");
  let meminfo = String::from_utf8(meminfo.output().expect("Failed to run `cat /proc/meminfo`").stdout).expect("");

  let meminfo_lines = meminfo.split('\n');

  let mut total_memory = 0;
  let mut used_memory = 0;

  for line in meminfo_lines {
    if line.starts_with("MemTotal") {
      let split_line = line.split(' ').collect::<Vec<_>>();
      total_memory = split_line[split_line.len()-2].parse::<i32>().unwrap() / 1024;
    }
    if line.starts_with("MemAvailable") {
      let split_line = line.split(' ').collect::<Vec<_>>();
      used_memory = total_memory - (split_line[split_line.len()-2].parse::<i32>().unwrap() / 1024);
    }
  }

  let mut info: Vec<String> = Vec::new();
    info.push(format!("OS NAME    : {}", pretty_name));
    info.push(format!("KERNEL     : {}", kernel_name));
    info.push(format!("HOSTNAME   : {}", hostname));
    info.push(format!("USER       : {}", user_name));
    info.push(format!("UPTIME     : {}", uptime));
    info.push(format!("SHELL      : {}", shell_name));
    info.push(format!("PACK. MAN. : {}", package_manager));
    info.push(format!("HARDWARE   : {}", vendor_and_model));
    info.push(format!("CPU        : {}x {}", cpu_count, cpu_name));
  for gpu in vga_info {
    info.push(format!("GPU        : {}", gpu));
  }
    info.push(format!("MEMORY     : {} / {} MiB", used_memory, total_memory));

  for x in info {
    println!("{}", x);
  }

}


fn index_first_character(text: String, query: char) -> usize {
  let mut index_of_occurence: usize = 0;
  for (index, char) in text.clone().as_bytes().iter().enumerate() {
    if String::from_utf8(vec![*char]).expect("idk") == String::from(query) {
    index_of_occurence = index;
    break
    }
  }
  index_of_occurence
}
