package problem

import (
	"bojexec/pkg/crawl"
	"bojexec/pkg/util/log"
	"fmt"
	"github.com/PuerkitoBio/goquery"
	"net/http"
	"strings"
)

var (
	baseURL = "https://www.acmicpc.net/problem/%d"
)

func Get(id uint) Problem {
	url := fmt.Sprintf(baseURL, id)

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.GetLogger().Error("Failed to create request", "trace", err)
	}
	req.Header.Set("User-Agent", crawl.Linux)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.GetLogger().Error("Failed to fetch URL", "trace", err)
	}
	defer resp.Body.Close()

	doc, err := goquery.NewDocumentFromReader(resp.Body)
	if err != nil {
		log.GetLogger().Error("Error loading HTTP response body: %v", "trace", err)
	}

	problemSelection := doc.Find("#problem_description")
	problem, err := problemSelection.Html()
	if err != nil {
		problem = problemSelection.Text()
	}

	inputSelection := doc.Find("#problem_description")
	input, err := inputSelection.Html()
	if err != nil {
		input = inputSelection.Text()
	}

	outputSelection := doc.Find("#problem_input")
	output, err := outputSelection.Html()
	if err != nil {
		output = outputSelection.Text()
	}

	var testcases []TestCase
	doc.Find("[id^='sample-input-']").Each(func(index int, selection *goquery.Selection) {
		testcases = append(testcases, TestCase{
			Input:  selection.Text(),
			Output: doc.Find(fmt.Sprintf("#sample-output-%d", index+1)).Text(),
		})
	})

	return Problem{
		ID:    id,
		Title: strings.TrimSpace(doc.Find("#problem_title").First().Text()),
		Description: Description{
			Problem: problem,
			Input:   input,
			Output:  output,
		},
		Example: testcases,
	}
}
