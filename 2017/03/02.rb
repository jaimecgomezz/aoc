DIRECTIONS = {
  'E' => 'N',
  'N' => 'W',
  'W' => 'S',
  'S' => 'E',
}

SQUARES_VALUES = {
  '0:0' => 1,
}
def next_square(x, y, direction)
  nx = x
  ny = y

  case direction
  when 'E'
    nx += 1
  when 'N'
    ny += 1
  when 'W'
    nx -= 1
  when 'S'
    ny -= 1
  end

  [nx, ny]
end

def move(x, y, direction)
  x, y = next_square x, y, direction
  nx, ny = next_square x, y, DIRECTIONS[direction]
  direction = DIRECTIONS[direction] unless SQUARES_VALUES.key? "#{ny}:#{nx}"

  [x, y, direction]
end

def sum(x, y)
  no = [x, y + 1]
  ne = [x + 1, y + 1]
  es = [x + 1, y]
  se = [x + 1, y - 1]
  so = [x, y - 1]
  sw = [x - 1, y - 1]
  we = [x - 1, y]
  nw = [x - 1, y + 1]

  total = 0
  [no, ne, es, se, so, sw, we, nw].each do |direction|
    x, y = direction
    square = "#{y}:#{x}"
    value = SQUARES_VALUES[square]

    total += value unless value.nil?
  end

  total
end

def spiral(target)
  x = 1
  y = 0
  direction = 'N'

  for n in 1..target do
    value = sum x, y
    square = "#{y}:#{x}"
    SQUARES_VALUES[square] = value

    if value > target
      puts value
      break
    end

    x, y, direction = move x, y, direction
  end
end

spiral 368_078
