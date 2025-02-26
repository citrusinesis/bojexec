package problem

type Description struct {
	Problem string
	Input   string
	Output  string
}

type TestCase struct {
	Input  string
	Output string
}

type Problem struct {
	ID          uint
	Title       string
	Description Description
	Example     []TestCase
}
