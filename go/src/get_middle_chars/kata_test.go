package kata

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"
	"testing"
)

var _ = Describe("GetMiddle", func() {
	It("Tests", func() {
		Expect(GetMiddle("test")).To(Equal("es"))
		Expect(GetMiddle("testing")).To(Equal("t"))
		Expect(GetMiddle("middle")).To(Equal("dd"))
		Expect(GetMiddle("A")).To(Equal("A"))
	})
})

func Test(t *testing.T) {
	RegisterFailHandler(Fail)
	RunSpecs(t, "Get Middle Chars")
}
