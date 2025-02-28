# Bojexec (Baekjoon Online Judge Executor)

## Overview

백준 잘 풀고 싶어요 입출력 예제 복사하고 비교하는거 귀찮아요 누군가 해줘요

## Project Structure

```text
bojexec
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
    - [X] Send Request to baekjoon
    - [X] Parse problem document
    - [X] Abstract problem entity
    - [ ] Parse checker metadata (Timeout, Memory Usage)
- [X] FS
    - [X] Create directory structure to save problem data
    - [X] Read solution code from fs
    - [X] Parse file extension to determine language
    - [X] Save crawled test cases into `test-cases.json` (For caching & append custom test cases)
    - [X] Save problem description to Markdown
- [ ] Runner
    - [ ] Get Docker image & Run container
    - [ ] Configurable resource by fetching metadata from online
        - [ ] Timeout
        - [ ] Memory Usage
    - [ ] Output validator to check results of tests ([Problem style docs](https://help.acmicpc.net/problem/style))
        - [ ] ICPC Style
        - [ ] Subtask
        - [ ] Score
        - [ ] Implement Function
        - [ ] Interactive
        - [ ] 2 Step
        - [ ] Check whole case (Same as ICPC style)
        - [ ] Implement Class
    - DomJudge? Or Language Per Image
        - [DomJudge Docker Image](https://hub.docker.com/r/domjudge/domserver/)
        - [선배님의 채점서버 운영 수기](https://docs.google.com/document/d/1hzp67Ql6v2tEx6eM-SXVxrL2slnJblls/edit?tab=t.0)
- [ ] Config
    - [ ] Define config schema
    - [ ] Config parser (YAML, TOML, JSON, ...)
    - [ ] Save default configuration (`$HOME/config/.bojexec/default.yaml`)
    - [ ] Config file per problem
- [ ] Command
    - [ ] Fetch problem (`boj fetch 1000`)
    - [ ] Run/Validate test cases (`boj run 1000 --solution main.rs`)

## Further Improvements

- Login & Submit
    - login: `boj login`
    - submit: `boj submit 1000 --solution main.rs`
- Problem Indexer by algorithm
- Blog Helper
    - Review file builder (Markdown, HTML, ...)
    - Post to CMS (Hugo, Jekyll, ...)
- GUI