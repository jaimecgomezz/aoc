# --- Part Two ---
#
# Out of curiosity, the debugger would also like to know the size of the loop:
# starting from a state that has already been seen, how many block
# redistribution cycles must be performed before that same state is seen again?
#
# In the example above, 2 4 1 2 is seen again after four cycles, and so the
# answer in that example would be 4.
#
# How many cycles are in the infinite loop that arises from the configuration in
# your puzzle input?

def redistribute(banks)
  blocks = banks.max
  index = banks.index(blocks)

  banks[index] = 0
  for block in 1..blocks do
    nindex = (index + block) % banks.size
    banks[nindex] = banks[nindex] + 1
  end

  banks
end

def deep_reallocations(input)
  banks = input.split(' ').map(&:to_i)
  redistributed = banks

  steps = 0
  repeated = nil
  repeated_at = 0
  seen = Set.new([banks.join('-')])

  loop do
    redistributed = redistribute(banks)
    pattern = redistributed.join('-')

    steps += 1

    if seen.include?(pattern)
      if repeated.nil?
        repeated = pattern
        repeated_at = steps
      elsif repeated == pattern
        break
      end
    end

    seen << pattern
  end

  steps - repeated_at
end

puts deep_reallocations('0 2 7 0')
puts deep_reallocations('5 1 10 0 1 7 13 14 3 12 8 10 7 12 0 6')
