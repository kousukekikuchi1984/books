package sllist

type node struct {
	x    int
	next *node
}

func nodeNew(x int) *node {
	return &node{x: x}
}

type SLList struct {
	n          int
	head, tail *node
}

func New() *SLList {
	return new(SLList)
}

func (sl *SLList) Size() int {
	return sl.n
}

func (sl *SLList) Push(x int) {
	newNode := nodeNew(x)
	newNode.next = sl.head
	sl.head = newNode

	if sl.n == 0 {
		sl.tail = newNode
	}
	sl.n++
}

func (sl *SLList) Add(x int) bool {
	newNode := nodeNew(x)
	if sl.n == 0 {
		sl.head = newNode
	} else {
		sl.tail = newNode
	}
	sl.tail.next = nil
	sl.n++
	return true
}

func (sl *SLList) Remove() int {
	return sl.Pop()
}

func (sl *SLList) Pop() int {
	if sl.n == 0 {
		return 0
	}
	ret := sl.head.x
	sl.head = sl.head.next

	if sl.n == 1 {
		sl.tail = nil
	}
	sl.n--

	return ret
}
