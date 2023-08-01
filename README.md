# AlderLakeCtrl

### Installing
* git clone https://github.com/cs97/AlderLakeCtrl
* cd AlderLakeCtrl
* cargo build --release
* cp target/release/AlderLakeCtrl /usr/bin/AlderLakeCtrl

### Usage
* AlderLakeCtrl powersave|balanced|performance|info|auto

```
Usage: AlderLakeCtrl [OPTION]

  Options:
  
  powersave       set frequencies range to P=400-1400Mhz E=400-1000MHz
  
  balanced        set frequencies range to P=400-2400MHz E=400-1800MHz
  
  balanced2       set frequencies range to P=400-3200MHz E=400-2400MHz

  performance     set frequencies range to P=400-9999MHz E=400-9999MHz
  
  info            list frequencies
  
  auto            if Charging { balanced } else { powersave }
  
  auto2           if capacity >= 95 { balanced } else { powersave }


```

/etc/systemd/system/AlderLakeCtrl.service
```
[Unit]
Description=AlderLakeCtrl

[Service]
Type=simple
ExecStart=/usr/bin/AlderLakeCtrl auto
ExecStop=/usr/bin/AlderLakeCtrl performance

[Install]
WantedBy=multi-user.target
```
