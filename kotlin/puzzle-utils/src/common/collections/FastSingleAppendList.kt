package common.collections

class FastSingleAppendList<out T>(val headList: List<T>, val tailElement: T) : List<T> {
    private val delegate: List<T> by lazy {
        ArrayList<T>(headList.size + 1).apply {
            if (headList is FastSingleAppendList) headList.fill(this) else addAll(headList)
            add(tailElement)
        }
    }

    override val size = headList.size + 1

    override fun contains(element: @UnsafeVariance T) = tailElement == element || headList.contains(element)

    override fun containsAll(elements: Collection<@UnsafeVariance T>) = elements.all { contains(it) }

    override fun get(index: Int) = delegate[index]

    override fun indexOf(element: @UnsafeVariance T) = delegate.indexOf(element)

    override fun isEmpty() = false

    override fun iterator() = delegate.iterator()

    override fun lastIndexOf(element: @UnsafeVariance T) =
        if (element == tailElement) size - 1 else headList.lastIndexOf(element)

    override fun listIterator() = delegate.listIterator()

    override fun listIterator(index: Int) = delegate.listIterator(index)

    override fun subList(fromIndex: Int, toIndex: Int) = delegate.subList(fromIndex, toIndex)

    internal fun fill(filler: MutableList<@UnsafeVariance T>) {
        if (headList is FastSingleAppendList) headList.fill(filler) else filler.addAll(headList)
        filler.add(tailElement)
    }
}