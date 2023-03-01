
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use std::{thread, time::Duration};

fn set_min_mhz(c: usize, mhz: usize){
  let path: String = format!("{}{}{}", "/sys/devices/system/cpu/cpu", c, "/cpufreq/scaling_min_freq");
  let mut f = File::create(path).unwrap();
  let value: String = format!("{}000", mhz);
  f.write_all(value.as_bytes()).unwrap();
}

fn set_max_mhz(c: usize, mhz: usize){
  let path: String = format!("{}{}{}", "/sys/devices/system/cpu/cpu", c, "/cpufreq/scaling_max_freq");
  let mut f = File::create(path).unwrap();
  let value: String = format!("{}000", mhz);
  f.write_all(value.as_bytes()).unwrap();
}

fn read_mhz(s: String) -> String {
  let mut data = fs::read_to_string(s).unwrap();
  data = data[..data.len()-1].to_string();
  return (data.parse::<usize>().unwrap()/1000).to_string()+"MHz";
}

fn core_info(core: &usize) -> String {
  let cur = read_mhz(format!("{}{}{}", "/sys/devices/system/cpu/cpu", core, "/cpufreq/scaling_cur_freq"));
  let min = read_mhz(format!("{}{}{}", "/sys/devices/system/cpu/cpu", core, "/cpufreq/scaling_min_freq"));
  let max = read_mhz(format!("{}{}{}", "/sys/devices/system/cpu/cpu", core, "/cpufreq/scaling_max_freq"));
  return format!("{} \t{}\t\t{}\t\t{}", core, cur, min, max);
}

fn cpu_info(p: usize, e: usize) -> () {
  println!("Core\t\tcurr\t\tmin\t\tmax");

  // P-Cores
  if p > 0 {
    println!("");
    for x in 0..p {
      println!("Core P{}",core_info(&x));
    }
  }
  
  // E-Cores
  if e > 0 {
    println!("");
    for x in p..p+e {
      println!("Core E{}", core_info(&x));
    }
  }
}

fn powersave(p: usize, e: usize) {
  for x in 0..p {
    set_min_mhz(x, 400);
    set_max_mhz(x, 1400);
  }
  for x in p..p+e {
    set_min_mhz(x, 400);
    set_max_mhz(x, 1400);
  }
}

fn balanced(p: usize, e: usize) {
  for x in 0..p {
    set_min_mhz(x, 400);
    set_max_mhz(x, 2400);
  }
  for x in p..p+e {
    set_min_mhz(x, 400);
    set_max_mhz(x, 1800);
  }
}

fn balanced2(p: usize, e: usize) {
  for x in 0..p {
    set_min_mhz(x, 400);
    set_max_mhz(x, 3200);
  }
  for x in p..p+e {
    set_min_mhz(x, 400);
    set_max_mhz(x, 2400);
  }
}

fn performance(p: usize, e: usize) {
  for x in 0..p {
    set_min_mhz(x, 400);
    set_max_mhz(x, 9999);
  }
  for x in p..p+e {
    set_min_mhz(x, 400);
    set_max_mhz(x, 9999);
  }
}

fn auto_mode(p: usize, e: usize) -> () {
  let mut last_status = "empty".to_string();

  loop {
    thread::sleep(Duration::from_millis(400));
    let mut status = fs::read_to_string("/sys/class/power_supply/BAT1/status").unwrap();
    status = status[..8].to_string();

    if status == "Charging" && last_status != status {
      last_status = status;
      balanced(p, e);
    } else if last_status != status {
      last_status = status;
      powersave(p, e);
    }
  }

}

fn auto_mode2(p: usize, e: usize) -> () {
  let mut last_cap = 100;

  loop {
    thread::sleep(Duration::from_millis(400));
    let capacity = fs::read_to_string("/sys/class/power_supply/BAT1/capacity").unwrap();
    let cap: usize = capacity[..capacity.len() - 1].parse::<usize>().unwrap();

    if cap >= 90 && last_cap != cap {
      last_cap = cap;
      balanced2(p, e);
    } else if last_cap != cap {
      last_cap = cap;
      powersave(p, e);
    }
  }

}

fn core_count() -> usize {
  let mut cores = 1;
  loop {
    let s = format!("/sys/devices/system/cpu/cpu{}", cores);
    if Path::new(&s).is_dir() {
      cores += 1;
      continue;
    } else {
      break;
    };
  };
	//cores += 1;
	return cores;
}

fn print_usage() -> () {
	//println!("usage:");
	//println!("\tAlderLakeCtrl {}", "powersave|balanced|performance|info|auto\n");
	println!("Usage: AlderLakeCtrl [OPTION]\n
		\n\tOptions:
		\n\tpowersave\tset frequencies range to 400-1400Mhz
		\n\tbalanced\tset frequencies range to P=400-2400MHz E=400-1800MHz
		\n\tbalanced2\tset frequencies range to P=400-3200MHz E=400-2400MHz
		\n\tperformance\tset frequencies range to P=400-9999MHz E=400-9999MHz
		\n\tinfo\t\tlist frequencies
		\n\tauto\t\tif Charging {{ balanced }} else {{ powersave }}
		\n\tauto2\t\tif capacity >= 90 {{ balanced2 }} else {{ powersave }}
    ");

}

fn switch_case(s: &str, p:usize, e: usize) -> () {
  match s {
    "powersave" => powersave(p, e),
    "balanced" => balanced(p, e),
    "balanced2" => balanced2(p, e),
    "performance" => performance(p, e),
    "info" => cpu_info(p, e),
    "auto" => auto_mode(p, e),
    "auto2" => auto_mode2(p, e),
    _ => print_usage(),
  }
}

fn main() {

  let mut p = 0;
  let mut e = 0;

  let cores = core_count();

  //p = cores;

  // 1220P
  if cores == 12 {
    p = 4;
    e = 8;
  }

  // 1240P & 1260P
  if cores == 16 {
    p = 8;
    e = 8;
  }

  // 1280P
  if cores == 20 {
    p = 12;
		e = 8;
  }

  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    print_usage();
  } else {
    let arg = &args[1].as_str();
    switch_case(arg, p, e);
  }
}
