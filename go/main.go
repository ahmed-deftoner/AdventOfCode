package main

import (
	"os"
	"strconv"
	"strings"
)

func checkIncreasing(x []int) int {
	count := 0
	for i := 1; i < len(x); i++ {
		if x[i-1] < x[i] {
			count++
		}
	}
	return count
}

func checkWindow(x []int) int {
	sumArr := []int{}
	sum := 0
	counter := 0
	i := 0
	for i < len(x) {
		if counter < 3 {
			sum += x[i]
			counter++
			i++
		} else if counter == 3 {
			sumArr = append(sumArr, sum)
			counter = 0
			sum = 0
			i -= 2
		}
	}
	return checkIncreasing(sumArr)
}

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
	println(checkWindow(nums))
}
