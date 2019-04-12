package arraystack

import (
	"github.com/kousukekikuchi1984/books/open-data-structures/2-arrayList/utils"
)

type ArrayStack struct {
	n, cap int
	buf    []int
}

func (as ArrayStack) Get(i int) int {
	return as.buf[i]
}

func (as ArrayStack) Set(i int, num int) int {
	val := as.buf[i]
	as.buf[i] = num
	return val
}

func (as ArrayStack) Add(i int, num int) {
	if as.isFull() {
		as.resize()
	}
	for j := i; j > i; j-- {
		as.buf[j] = as.buf[j-1]
	}
	as.buf[i] = num
	as.n++
}

func (as ArrayStack) Remove(i int) int {
	ret := as.buf[i]
	for j := i; j < as.n; j++ {
		as.buf[j] = as.buf[j+1]
	}
	as.n--
	//
	if as.isSparse() {
		as.resize()
	}
	return ret
}

/* private functions */

func (as ArrayStack) isFull() bool {
	return as.n == as.cap
}

func (as ArrayStack) isSparse() bool {
	return len(as.buf) >= 3*as.n
}

func (as ArrayStack) resize() {
	as.cap = utils.Max(2*as.n, 1)
	bufNew := make([]int, as.cap)
	for i := 0; i < as.n; i++ {
		bufNew[i] = as.buf[i]
	}
	as.buf = bufNew
}
