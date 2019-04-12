package dualarraydequeue

import (
	"github.com/kousukekikuchi1984/books/2-arrayList/arraystack"
)

type DualArrayDequeue struct {
	n           int
	front, back arraystack.ArrayStack
}

func (dad DualArrayDequeue) Size() int {
	return dad.front.Size() + dad.back.size()
}

func (dad DualArrayDequeue) Get(i int) int {
	if i < dad.front.Size() {
		return dad.front.Get(dad.front.Size() - i - 1)
	} else {
		return dad.back.Get(i - dad.front.size())
	}
}

func (dad DualArrayDequeue) Set(i int, x int) {
	if i < dad.front.Size() {
		return dad.front.Set(dad.front.Size()-i-1, x)
	} else {
		return dad.back.Set(i-dad.front.Size(), x)
	}
}

func (dad DualArrayDequeue) Add(i int, x int) {
	if i < dad.front.Size() {
		dad.front.Add(front.Size())
	} else {
		dad.back.Add(i-dad.front.Size(), x)
	}
	dad.balance()
	dad.n++
}

func (dad DualArrayDequeue) Remove(i int) {
	var ret int
	if frontSize := dad.front.Size(); i < frontSize {
		ret = dad.front.Remove(frontSize - i - 1)
	} else {
		ret = dad.back.Remove(i - frontSize)
	}
	dad.balance()
	dad.n--
	return ret
}

func (dad DualArrayDequeue) balance() {
	frontSize := dad.front.Size()
	backSize := dad.back.Size()
	//
	if 3*frontSize < backSize || 3*backSize < frontSize {
		n := dad.Size()

		frontSizeNew := n / 2
		frontNew := arraystack.New()
		for i := frontSizeNew - 1; i >= 0; i-- {
			frontNew.Push(dad.Get(i))
		}
		//
		backSizeNew := n - frontSizeNew
		backNew := arraystack.New()
		for i := 0; i < backSizeNew; i++ {
			backNew.Push(dad.Get(frontSizeNew + i))
		}
		dad.front = frontNew
		dad.back = backNew
	}
}
