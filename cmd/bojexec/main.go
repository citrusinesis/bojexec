package main

import "fmt"

// Build information. These will be populated by ldflags during build
var (
	Name      = "example" // Change this to your app name
	Version   = "dev"
	Commit    = "none"
	BuildTime = "unknown"
)

func main() {
	fmt.Println("Entrypoint of BOJEXEC", Name, Version, Commit, BuildTime)
}
