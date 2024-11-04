DIRECTIONS = {
  'E' => 'N',
  'N' => 'W',
  'W' => 'S',
  'S' => 'E',
}

SQUARES_VALUES = {}
VALUES_SQUARES = {}

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

def spiral(target)
  x = 0
  y = 0
  direction = 'E'

  for n in 1..target do
    square = "#{y}:#{x}"
    SQUARES_VALUES[square] = target
    VALUES_SQUARES[target] = square
    x, y, direction = move x, y, direction
  end

  square = VALUES_SQUARES[target]
  x, y = square.split(':').map(&:to_i)
  puts (x + y).abs
end

spiral 368_078
