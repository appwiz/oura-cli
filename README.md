# oura-cli
Get sleep scores by date from the Oura API.

## Commands
```shell
% oura-cli --help
Usage: oura-cli [COMMAND]

Commands:
configure
show
score
help       Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
```
### Configure the CLI
```shell
% oura-cli configure --help
Usage: oura-cli configure --oura-token <OURA_TOKEN>

Options:
-o, --oura-token <OURA_TOKEN>
-h, --help                     Print help
```

### Display CLI settings
```shell
% oura-cli show --help
Usage: oura-cli show

Options:
-h, --help  Print help
```

### Get the sleep score for a specific date range in a particular format
```shell
% oura-cli score --help
Usage: oura-cli score [OPTIONS] --start-date <START_DATE> --end-date <END_DATE>

Options:
-s, --start-date <START_DATE>
-e, --end-date <END_DATE>
-o, --output-format <OUTPUT_FORMAT>  [default: text]
-h, --help                           Print help
```

## Examples
### Set the Oura API token 
```shell
% oura-cli configure --oura-token EXAMPLETOKEN
Oura token has been configured.
```
### Show the current Oura API token
```shell
% oura-cli show
Oura token: EXAMPLETOKEN
```

### Get the sleep score for a specific date range
```shell
% oura-cli score -s "2024-06-01" -e "2024-06-02"
Date: "2024-06-01", Sleep score: 67
Date: "2024-06-02", Sleep score: 59
```

### Get the sleep score for a specific date range in JSON format
```shell
% oura-cli score -s "2024-06-01" -e "2024-06-02" -o json
[{"date":"2024-06-01","score":67},{"date":"2024-06-02","score":59}]
```
