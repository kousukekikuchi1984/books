package rootisharraystack

import (
	"math"
)

type RootishArrayStack struct {
	n      int
	blocks [][]int
}

func New() RootishArrayStack {
	return RootishArrayStack{}
}

func i2b(i int) int {
	db := (-3.0 + math.Sqrt(float64(9+8*i))) / 2.0
	return int(math.Ceil(db))
}

func (ras RootishArrayStack) Get(i int) int {
	b := i2b(i)
	j := i - b*(b+1)/2
	return ras.blocks[b][j]
}

func (ras RootishArrayStack) Set(i int, v int) int {
	b := i2b(i)
	j := i - b*(b+1)/2
	y := ras.blocks[b][j]
	ras.blocks[b][j] = v
	return y
}

func (ras RootishArrayStack) Add(i int, v int) {
	r := len(ras.blocks)
	if r*(r+1)/2 < ras.n+1 {
		ras.grow()
	}
	ras.n++
	for j := ras.n - 1; j > i; j-- {
		ras.Set(j, ras.Get(j-1))
	}
	ras.Set(i, v)
}

func (ras RootishArrayStack) grow() {
	ras.blocks = append(ras.blocks, make([]int, len(ras.blocks)+1))
}
