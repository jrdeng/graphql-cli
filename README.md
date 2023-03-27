# graphquery

A simple cli tool to make graphql request to specified server.

## Usage

Command and options:
```
Usage: graphquery.exe [OPTIONS] --url <API_URL> --token <TOKEN>

Options:
  -u, --url <API_URL>         URL of GraphQL API. for example: https://<your-server.com>/api/graphql
  -t, --token <TOKEN>         Token to access the API
  -f, --file <QUERY_FILE>     File that contains the query string. one of '-f' and '-q' must be set
  -q, --query <API_URL>       Query string. '-f' will be omitted if this is set
  -o, --output <OUTPUT_FILE>  Output file to store the query result. The result is echo to std out by default
  -h, --help                  Print help
```

### Example

Query with specified string:
```
cargo run -- --url=https://api.github.com/graphql --token=${YOUR_TOKEN} --query="query{viewer {login}}"
```


Query can be in a file:
```
cargo run -- --url=https://api.github.com/graphql --token=${YOUR_TOKEN} --file=graphql/github_issues.graphql --output=output.json
```
