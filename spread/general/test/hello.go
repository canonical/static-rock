package main

import (
	"fmt"
	"os"
)

func main() {
	if len(os.Args) > 1 {
		fmt.Println("Hello from Go:", os.Args[1])
	} else {
		fmt.Println("Hello from Go!")
	}
	os.Exit(0)
}
