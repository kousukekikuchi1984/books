package arraydequeue

import (
	"github.com/books/open_data_structures/2-arraylist/utils"
)

type ArrayDequeue struct {
	i, n, cap int
	buf       []int
}

func (ad ArrayDequeue) Get(i int) int {
	return ad.buf[(ad.i+i)%ad.cap]
}

func (ad ArrayDequeue) Set(i int, v int) int {
	ret := ad.Get(i)
	ad.buf[(ad.i+i)%ad.cap] = v
	return ret
}

func (ad ArrayDequeue) Add(i int, v int) {
	if ad.isFull() {
		ad.resize()
	}
	as.buf[(as.i+as.n)%as.cap] = v
	as.n++
}

func (ad ArrayDequeue) Remove(i int) {
	ret := ad.buf[(ad.i+i)%ad.cap]
	if i < ad.n/2 {
		for k := i; k > 0; k-- {
			ad.buf[(ad.i+k)%ad.cap] = ad.buf[(ad.i + k - 1) % ad.cap]
		}
		ad.i = (ad.i + 1) % ad.cap
	} else {
		for (k :=i; k < n-1; k++) {
			ad.buf[ (ad.i+k)%ad.cap  = ad.buf[ (ad.i+k-1) % ad.cap ]
		}
	}
	n--
	if ad.isSparse() {
		ad.resize()
	}
	return ret
}

func (ad ArrayDequeue) isFull() bool {
	return ad.n == ad.cap
}

func isSparse() {
	return len(as.buf) >= 3*as.n
}

func (ad *ArrayDequeue) resize() {
	capNew := utils.Max(2*ad.n, 1)
	bufNew := make([]int, capNew)

	for i := 0; i < ad.n; i++ {
		bufNew[i] = ad.buf[(ad.i+i)%ad.cap]
	}
	ad.buf = bufNew
	ad.cap = capNew
	ad.i = 0
}
