package main

type node struct {
	x int
	prev, next *node
}

type DLList struct {
	n int
	dummy *node
}

func nodeNew(x int) *node {
	return &node{x: x}
}

func New() *DLList {
	dummy := new(node)
	dummy.next = dummy
	dummy.prev = dummy
	return &DLList{dummy: dummy}
}

func (l *DLList)getNode(i int) *node {
	var p *node
	if i < l.n/2 {
		p = l.dummy.next
		for j := 0; j < i; j++ {
			p = p.next
		}
	} else {
		p = l.dummy
		for j := l.n; j > i; j -- {
			p = p.prev
		}
	}
	return p
}

func (l *DLList)get(i int) int {
	return l.getNode(i).x
}

func (l *DLList)set(i int, x int) int {
	u := l.getNode(i)
	ret := u.x
	u.x = x
	return ret
}

func (l *DLList)addBefore(w *node, x int) *node {
	u := nodeNew(x)
	u.prev = w.prev
	u.next = w
	u.next.prev = u
	u.prev.next = u
	l.n++
	return u
}

func (l *DLList)Add(i int, x int) {
	l.addBefore( l.getNode(i), x )
}

func (l *DLList)remove(w *node) {
	w.prev.next = w.next
	w.next.prev = w.prev
	w = nil
	l.n--
}




