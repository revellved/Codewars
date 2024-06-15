package kata

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
	"math"
	"testing"
)

var _ = Describe("Example test", func() {
	It("Example test case", func() {
		Expect(FindOutlier([]int{2, 6, 8, -10, 3})).To(Equal(3))
		Expect(FindOutlier([]int{206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781})).To(Equal(206847684))
		Expect(FindOutlier([]int{math.MaxInt32, 0, 1})).To(Equal(0))
	})
})

func Test(t *testing.T) {
	RegisterFailHandler(Fail)
	RunSpecs(t, "Find Outlier")
}
