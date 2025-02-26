package log

import (
	"log/slog"
	"os"
	"sync"
)

var (
	instance *slog.Logger
	once     sync.Once
)

func GetLogger() *slog.Logger {
	once.Do(func() {
		instance = slog.New(charmHandler(os.Stdout))
	})
	return instance
}
