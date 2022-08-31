function Dump(o)
  if type(o) == 'table' then
    local s = '{ '
    for k, v in pairs(o) do
      if type(k) ~= 'number' then k = '"' .. k .. '"' end
      s = s .. '[' .. k .. '] = ' .. Dump(v) .. ','
    end
    return s .. '} '
  else
    return tostring(o)
  end
end

function Shuffle(tbl)
  for i = #tbl, 2, -1 do
    local j = math.random(i)
    tbl[i], tbl[j] = tbl[j], tbl[i]
  end
  return tbl
end

local MIN_ROOM_WIDTH = 20
local MIN_ROOM_HEIGHT = 20

local MIN_INNER_ROOM_WIDTH = 6
local MIN_INNER_ROOM_HEIGHT = 6

return function(width, height)
  -- Make a root room.
  local root = {
    -- parent = nil,
    x = 0,
    y = 0,
    width = width,
    height = height,
    children = {}
  }

  -- Divide the root room into smaller rooms via BSP.
  local function divide_room(room)
    if room.width < 2 * MIN_ROOM_WIDTH and room.height < 2 * MIN_ROOM_HEIGHT then
      return
    end

    local function slice(min, size)
      local pivot = math.random(min, size - min)
      return {
        {
          offset = 0,
          size = pivot
        },
        {
          offset = pivot,
          size = size - pivot
        }
      }
    end

    local direction = math.random(0, 1)

    if direction == 0 then
      if room.width < 2 * MIN_ROOM_WIDTH then
        direction = 1 - direction
      end
    end

    if direction == 1 then
      if room.height < 2 * MIN_ROOM_HEIGHT then
        direction = 1 - direction
      end
    end

    if direction == 0 then
      local children = slice(MIN_ROOM_WIDTH, room.width)

      table.insert(room.children, {
        -- parent = room,
        x = room.x + children[1].offset,
        y = room.y,
        width = children[1].size,
        height = room.height,
        children = {}
      })
      table.insert(room.children, {
        -- parent = room,
        x = room.x + children[2].offset,
        y = room.y,
        width = children[2].size,
        height = room.height,
        children = {}
      })

      for _, child in ipairs(room.children) do
        divide_room(child)
      end
    else
      local children = slice(MIN_ROOM_HEIGHT, room.height)

      table.insert(room.children, {
        -- parent = room,
        x = room.x,
        y = room.y + children[1].offset,
        width = room.width,
        height = children[1].size,
        children = {}
      })
      table.insert(room.children, {
        -- parent = room,
        x = room.x,
        y = room.y + children[2].offset,
        width = room.width,
        height = children[2].size,
        children = {}
      })

      for _, child in ipairs(room.children) do
        divide_room(child)
      end
    end
  end

  divide_room(root)

  local leafs = {}

  local function collect_leafs(room)
    if #room.children == 0 then
      table.insert(leafs, room)
    else
      for _, child in ipairs(room.children) do
        collect_leafs(child)
      end
    end
  end

  collect_leafs(root)

  local rooms = {}

  for _, leaf in ipairs(leafs) do
    local inner_width = math.random(MIN_INNER_ROOM_WIDTH, leaf.width - 2)
    local inner_height = math.random(MIN_INNER_ROOM_HEIGHT, leaf.height - 2)
    local inner_x = math.random(leaf.x + 1, leaf.x + leaf.width - inner_width - 1)
    local inner_y = math.random(leaf.y + 1, leaf.y + leaf.height - inner_height - 1)

    table.insert(rooms, {
      x = inner_x,
      y = inner_y,
      width = inner_width,
      height = inner_height
    })
  end

  Shuffle(rooms)

  local map = {}

  for i = 1, width * height do
    map[i] = 0
  end

  for i, room in ipairs(rooms) do
    if #rooms / 2 < i then
      break
    end

    for x = room.x, room.x + room.width - 1 do
      for y = room.y, room.y + room.height - 1 do
        if x == room.x or x == room.x + room.width - 1 or y == room.y or y == room.y + room.height - 1 then
          map[1 + x + y * width] = 1
        else
          map[1 + x + y * width] = 2
        end
      end
    end
  end

  return {
    rooms = rooms,
    map = map
  }
end
