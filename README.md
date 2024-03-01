Purpose: improve the financial audit (accounting) workflow.  

<img src="./asset/stflow3.png" alt="ADA workflow" width="53%" height="auto">     

Add more functions between setup and report if needed. For instance,  

```
polars
duckdb
arrow
fusen
devtools
```

Setup:    
- Ubuntu 22.04.3 LTS on WSL2
- build the binary from source
- put it in the PATH
- export the env variable to the shell

[<img src="https://images.pexels.com/photos/20324868/pexels-photo-20324868/free-photo-of-a-small-house-sits-on-the-edge-of-a-lake.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1)" width="800" height="400"
/>](https://www.youtube.com/watch?v=sixT4Sqn_A8)


Usage:  

```
faudit help
faudit init
faudit new -c clientname -y 2024 -a "start a new audit project" -i
faudit report -c clientname -y 2024 -a "generate the final report" -i
faudit show tree
faudit show list
```

```
faproj/
├── box
│   ├── config.yml
│   └── stbox
│       └── box.R
├── config.json
└── job
    └── shanghai
        └── 2023
            ├── awp
            │   └── clean.R
            ├── doc
            ├── misc
            ├── pbc
            └── report
                ├── report.html
                ├── report.qmd
                └── report_files
```

