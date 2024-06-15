package kata

func FindOutlier(integers []int) int {
	odd := 0
	even := 0

	last_odd := 0
	last_even := 0

	for _, i := range integers {
		if i%2 == 0 {
			odd += 1
			last_odd = i
		} else {
			even += 1
			last_even = i
		}

		if odd >= 2 && even >= 1 {
			return last_even
		} else if even >= 2 && odd >= 1 {
			return last_odd
		}
	}
	return 0
}
