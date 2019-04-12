# Introduction

## Data Structure Interface

### Queue or FIFO Queue

First In First Out

* add(x)
* remove()

### Priority Queue

Queueから要素を削除する時に、最小のものを削除する

* add(x)
* remove()

### Stack or LIFO Queue

Last In First Out

* push(x)
* pop()

### Deque
双方向キュー 先頭と末尾を持った要素の列で、先頭または末尾に要素を追加できる

* addFirst()
* removeFirst()
* addLast()
* removeLast()

### List Interface

* size()
* get(i)
* set(i, x)
* add(i, x)
* remove(i)

### USet Interface

重複がなく順序がない要素の集まりの集合

* size()
* add(x)
* remove(x)
* find(x)

findは要素内にxがあればxを返す

### SSet Interface

順序づけされた要素の集合。

Interface自体はUSetと同じだが、findは、要素ないにx以上の最小の要素を見つける挙動

## 数学的背景

### 指数、対数、階乗

知っているので、無視

### 漸近記法

実行時間を推定する際に用いる大まかな時間の推定。基本的には、関数f(n)のnが十分に大きい時のことを考える


