# Bojexec (Baekjoon Online Judge Executor)

## Overview
백준 잘 풀고 싶어요 입출력 예제 복사하고 비교하는거 귀찮아요 누군가 해줘요

## Project Structure
```text
bojexec
├── build
├── cmd
│   └── bojexec     // Entrypoint
├── internal
│   ├── cmd         // CLI command
│   └── problem     // Problem domain
└── pkg
    ├── config      // Executor config parser (YAML, ...?)
    ├── crawl       // Problem page crawler
    ├── fs          // Read source code from filesystem
    ├── runner      // Code runner powered by docker
    └── util        // Util packages
```

## Checkpoint
- [X] Problem Crawler
  - [X] Send Request to baekjoon
  - [X] Parse problem document
  - [X] Abstract problem entity
- [ ] FS
  - [ ] Create directory structure to save problem data
  - [ ] Read solution code from fs
  - [ ] Parse file extension to determine language
  - [ ] Save crawled test cases into `test-cases.json` (For caching & append custom test cases)
- [ ] Runner
  - [ ] Get Docker image
  - [ ] Run container
  - [ ] Configurable resource by fetching metadata from online
    - [ ] Run Time
    - [ ] Memory Usage
  - [ ] Output validator to check results of tests
- [ ] Config
  - [ ] Define config schema
  - [ ] Config parser (YAML, TOML, JSON, ...)
  - [ ] Save default configuration (`$HOME/config/.bojexec/default.yaml`)
  - [ ] Config file per problem
- [ ] Command (cobra)
  - [ ] Fetch problem (`boj fetch 1000`)
  - [ ] Run/Validate test cases (`boj run 1000 --solution main.go`)

## Further Improvements
- Login & Submit
  - login: `boj login` -> redirect to oauth
- Problem Indexer by algorithm
- GUI