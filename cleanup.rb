def search(term) = `rg "((crate.+week)|(week.+crate)).+\\b#{term}\\b" ./this-week-in-rust/content/`

filtered = File.open('output.csv').each_line.select do |line|
  line
    .strip
    .split(?/)
    .last
    .then {|s| s[0...s.index(/[?#]/)]} # conveniently if index == nil it just...includes the end of the range
    .then do |term|
      search(term).empty?
    end
end

puts filtered.join