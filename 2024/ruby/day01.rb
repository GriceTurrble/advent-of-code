# frozen_string_literal: true

require 'benchmark'

def input_data
  current_day = File.basename(__FILE__, '.*')
  this_dir = File.dirname(File.expand_path(__FILE__))
  inputs_dir = File.join(File.dirname(this_dir), 'inputs')
  input_file = File.join(inputs_dir, "#{current_day}.txt")
  File.read(input_file)
end

def part1
  data = input_data

  lefts = []
  rights = []
  data.lines.each do |line|
    left, right = line.split(' ')
    lefts << left.to_i
    rights << right.to_i
  end
  lefts.sort!
  rights.sort!

  total = 0
  lefts.zip rights do |left, right|
    total += (left - right).abs
  end
  total
end

def part2
  data = input_data

  # 'lefts' is a hash map
  lefts = {}
  rights = {}

  data.lines.each do |line|
    left, right = line.split(' ')
    lefts[left.to_i] = lefts.fetch(left.to_i, 0) + 1
    rights[right.to_i] = rights.fetch(right.to_i, 0) + 1
  end

  total = 0
  lefts.each do |num, count|
    total += count * (num * rights.fetch(num, 0))
  end
  total
end

def main
  Benchmark.bm(7) do |x|
    x.report('p1:') { puts ">> Part 1: #{part1}" }
    x.report('p2:') { puts ">> Part 1: #{part2}" }
  end
end

main
