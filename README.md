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
- [ ] Problem Crawler
  - [ ] Send Request to baekjoon
  - [ ] Parse problem document
  - [ ] Abstract problem entity
- [ ] FS
  - [ ] Read source code from fs
  - [ ] Parse file extension to determine language
- [ ] Runner
  - [ ] Get Docker image
  - [ ] Run container
  - [ ] Configurable resource
  - [ ] Output validator to check test case (example input/output)
- [ ] Config
  - [ ] Config file parser
  - [ ] Save default configuration
  - [ ] Config file per problem
- [ ] Command (cobra)
  - [ ] Fetch problem
  - [ ] Run/Validate test cases
  - [ ] Submit

## Further Improvements
- Login & Submit
- Problem fetch and make directory structure
- Indexer (md? db? ..?)
- Web Pages