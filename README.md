# AlderLakeCtrl

### Installing
* cargo build --release
* cp target/release/AlderLakeCtrl /usr/bin/AlderLakeCtrl

### Usage
* AlderLakeCtrl powersave|balanced|performance|info|auto


* list frequencies
```
Usage: AlderLakeCtrl [OPTION]

  Options:
  
  powersave       set frequencies range to 400-1400Mhz
  
  balanced        set frequencies range to P=400-3000MHz E=400-2000MHz
  
  performance     set frequencies range to P=2000-9999MHz E=1200-9999MHz
  
  info            list frequencies

  auto            set powersave if Discharging else balanced

```
