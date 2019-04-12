package fastarraystack

import (
	"github.com/kousukekikuchi1984/books/open-data-structures/2-arrayList/utils"
)

type FastArrayStack struct {
	n, cap int
	buf    []int
}

func (fas FastArrayStack) Add(i int, value int) {
	if fas.isFull() {
		fas.resize()
	}
	//
	copy(fas.buf[i:fas.n-1], fas.buf[i+1:fas.n])
	fas.n--
	if fas.isSparse() {
		fas.resize()
	}
}

func (fas FastArrayStack) isFull() bool {
	return fas.n == fas.cap
}

func (fas FastArrayStack) isSparse() bool {
	return len(fas.buf) >= 3*fas.n
}

func (fas *FastArrayStack) resize() {
	fas.cap = utils.Max(3*fas.n, 1)
	bufNew := make([]int, fas.cap)
	copy(bufNew, fas.buf)
	fas.buf = bufNew
}
