package kata

func FindOutlier(integers []int) int {
	var odd, even *int
	for i, integer := range integers {
		if even != nil && odd != nil {
			if integer%2 == 0 {
				return *odd
			}
			return *even
		}
		if integer%2 == 0 {
			even = &integers[i]
		} else {
			odd = &integers[i]
		}
	}
	return integers[len(integers)-1]
}
