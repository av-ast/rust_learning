threads = []

10.times do |thread_id|
  threads << Thread.new do
    count = 0
    10_000_000.times do
      count += 1
    end
  end

  threads.each(&:join)
  puts "Thread #{thread_id} is finnished."
end
