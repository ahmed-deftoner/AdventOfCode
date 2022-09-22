package main

import (
	"os"
	"strconv"
	"strings"
)

func ReadFile(fileName string) (nums []int, err error) {
	body, err := os.ReadFile(fileName)
	if err != nil {
		return []int{}, err
	}

	Rawinput := strings.Split(string(body), "\n")
	inputs := []int{}
	for i := 0; i < len(Rawinput); i++ {
		value := Rawinput[i]
		if value == "" {
			continue
		}
		num, err := strconv.Atoi(value)
		if err != nil {
			return []int{}, err
		}
		inputs = append(inputs, num)
	}
	return inputs, nil
}

func main() {
	nums, err := ReadFile("E:/AdventOfCode/data.txt")
	if err != nil {
		println("failed to parse file")
	}
	println(len(nums))
}
