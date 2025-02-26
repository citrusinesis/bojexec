package log

import (
	"io"
	"log/slog"

	"github.com/charmbracelet/log"
)

func charmHandler(w io.Writer) slog.Handler {
	return log.NewWithOptions(w, log.Options{
		Level:           log.DebugLevel, // TODO: set level by env
		ReportCaller:    true,
		ReportTimestamp: true,
	})
}
