package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

func main() {
	// session := "53616c7465645f5f65cd3b465b5c1d429850affcf7c87af35c8267e3bd587f994a9140a7b6edaea8414b5608eba11fd0dc8ec8794f85b3804fd5acf5728c1f80"
	// input := http.GetPuzzleInput("2021", "1", session)
	dat, err := os.ReadFile("day1/input.txt")
	if err != nil {
		panic(err)
	}
	a := regexp.MustCompile(`\n\s\n`)
	b := regexp.MustCompile(`\n`)
	elves := a.Split(string(dat), -1)
	resultArray := []int{}
	for _, s := range elves {
		values := b.Split(s, -1)
		result := 0
		for _, numb := range values {
			numb = strings.Trim(numb, "\r")
			j, err := strconv.Atoi(numb)
			if err != nil {
				log.Fatalf("Got error %s", err.Error())
			}
			result += j
		}
		resultArray = append(resultArray, result)
	}
	sort.Ints(resultArray)
	lastNumb := len(resultArray)
	score := resultArray[lastNumb-1] + resultArray[lastNumb-2] + resultArray[lastNumb-3]
	fmt.Println(score)
	// prev := 0
	// score := 0
	// for i, s := range values {
	// 	fmt.Println(i, s)
	// 	j, err := strconv.Atoi(s)
	// 	if err != nil {
	// 		log.Fatalf("Got error %s", err.Error())
	// 	}

	// 	if j > prev {
	// 		score++
	// 	}
	// 	prev = j
	// }
	// fmt.Println(score - 1)
}
