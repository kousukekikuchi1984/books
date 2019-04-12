package arrayqueue

import (
	"github.com/kousukekikuchi1984/books/open-data-structures/2-arrayList/utils"
)

type ArrayQueue struct {
	n, cap, i int
	buf       []int
}

func (aq ArrayQueue) Add(x int) {
	if aq.isFull() {
		aq.resize()
	}
	//
	aq.buf[(aq.i+aq.n)%aq.cap] = x
	aq.n++
}

func (aq ArrayQueue) Remove() {
	res := aq[aq.i]
	aq.i = (aq.i + 1) % as.cap
	aq.n--
	if aq.isSparse() {
		as.resize()
	}
}

func (aq ArrayQueue) isSparse() {
	return len(aq.buf) >= 3*aq.n
}

func (aq ArrayQueue) isFull() {
	return aq.n == aq.cap
}

func (aq ArrayQueue) resize() {
	capNew := utils.Max(2*aq.n, 1)
	bufNew := make([]int, capNew)
	for i := 0; i < as.n; i++ {
		bufNew[i] = aq[(i+as.i)%as.cap]
	}
	as.buf = bufNew
	as.cap = capNew
	as.i = 0
}
