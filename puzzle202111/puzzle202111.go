package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	// session := "53616c7465645f5f65cd3b465b5c1d429850affcf7c87af35c8267e3bd587f994a9140a7b6edaea8414b5608eba11fd0dc8ec8794f85b3804fd5acf5728c1f80"
	// input := http.GetPuzzleInput("2021", "1", session)
	dat, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	values := strings.Fields(string(dat))
	prev := 0
	score := 0
	for i, s := range values {
		fmt.Println(i, s)
		j, err := strconv.Atoi(s)
		if err != nil {
			log.Fatalf("Got error %s", err.Error())
		}

		if j > prev {
			score++
		}
		prev = j
	}
	fmt.Println(score - 1)
}
