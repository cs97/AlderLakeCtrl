# AlderLakeCtrl

### Installing
* cargo build --release
* cp target/release/AlderLakeCtrl /usr/bin/AlderLakeCtrl

### Usage
* AlderLakeCtrl powersave|balanced|performance|info|auto

```
Usage: AlderLakeCtrl [OPTION]

  Options:
  
  powersave       set frequencies range to 400-1400Mhz
  
  balanced        set frequencies range to P=400-2400MHz E=400-2200MHz max 60W
  
  performance     set frequencies range to P=400-9999MHz E=400-9999MHz
  
  info            list frequencies

  auto            set powersave if Discharging else balanced

```
