# github-actions-execution-time-collector

[![CI](https://github.com/kazune-br/github-actions-execution-time-collector/actions/workflows/ci.yaml/badge.svg)](https://github.com/kazune-br/github-actions-execution-time-collector/actions/workflows/ci.yaml)
[![Release](https://github.com/kazune-br/github-actions-execution-time-collector/actions/workflows/release.yaml/badge.svg?branch=main)](https://github.com/kazune-br/github-actions-execution-time-collector/actions/workflows/release.yaml)

![execution-screen](./assets/execution-screen.gif)

# About
This project provides an easy solution for collecting github actions execution time from your command line. Outputs will be generated as csv files.

# Download Binary
This is one of the way to download assets. Please select appropriate one.
```bash
curl -LO $(curl -s https://api.github.com/repos/kazune-br/github-actions-execution-time-collector/releases/latest | jq -r ".assets[].browser_download_url" | fzf) 
```

![download](./assets/download.gif)

# Usage
```bash
./github-actions-execution-time-collector --help
```

```
github-actions-execution-time-collector 

USAGE:
    github-actions-execution-time-collector [OPTIONS]

OPTIONS:
        --from <from_date>       
    -h, --help                   Print help information
        --o <owner_name>         
        --r <repository_name>    
        --to <to_date> 
```

Before executing, you also need export github personal access token as below.
```bash
export GITHUB_TOKEN=paste your personal access token
```
