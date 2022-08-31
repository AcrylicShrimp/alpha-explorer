local Queue = {}

function Queue.new(queue)
  local q = { first = 0, last = -1 }
  setmetatable(q, { __index = queue })
  return q
end

function Queue.push_left(queue, value)
  local first = queue.first - 1
  queue.first = first
  queue[first] = value
end

function Queue.push_right(queue, value)
  local last = queue.last + 1
  queue.last = last
  queue[last] = value
end

function Queue.peek_left(queue)
  return queue[queue.first]
end

function Queue.peek_right(queue)
  return queue[queue.last]
end

function Queue.pop_left(queue)
  local first = queue.first
  if first > queue.last then error("queue is empty") end
  local value = queue[first]
  queue[first] = nil
  queue.first = first + 1
  return value
end

function Queue.pop_right(queue)
  local last = queue.last
  if queue.first > last then error("queue is empty") end
  local value = queue[last]
  queue[last] = nil
  queue.last = last - 1
  return value
end

return Queue
