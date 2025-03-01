package main

import (
	"bojexec/pkg/util/log"
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
}
