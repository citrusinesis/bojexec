package main

import (
	"bojexec/internal/problem"
	"bojexec/pkg/util/log"
	"fmt"
)

var (
	Name      = "bojexec"
	Version   = "dev"
	Commit    = "none"
	BuildTime = "unknown"
)

func main() {
	mainLog := log.GetLogger()

	mainLog.Debug("Entrypoint of BOJEXEC",
		"name", Name,
		"version", Version,
		"commit", Commit,
		"buildtime", BuildTime,
	)

	fmt.Println(problem.Get(1000))
}
