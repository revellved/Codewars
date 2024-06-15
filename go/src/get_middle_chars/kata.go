package kata

func GetMiddle(s string) string {
	if l := len(s) / 2; len(s)%2 == 0 {
		return s[l-1 : l+1]
	} else {
		return s[l : l+1]
	}
}
