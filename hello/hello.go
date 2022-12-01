package main

import (
	"advent2022/http"
	"fmt"
)

func main() {
	// Get a greeting message and print it.
	session := "53616c7465645f5f65cd3b465b5c1d429850affcf7c87af35c8267e3bd587f994a9140a7b6edaea8414b5608eba11fd0dc8ec8794f85b3804fd5acf5728c1f80"
	input := http.GetPuzzleInput("2021", "1", session)
	fmt.Println(input)
}
