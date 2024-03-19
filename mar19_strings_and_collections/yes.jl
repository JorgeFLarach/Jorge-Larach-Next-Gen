f = open("tempest.txt", "r")
s = read(f, String)
close(f)

words = split(s)

bigram_counts = Dict{Tuple{String, String}, Int}()
word_counts = Dict{String, Int}()

stop_words = ["the", "be", "to", "of", "and", "a", "in", "that", "have", "I", "it", "for", "not", "on", "with", "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we", "say", "her", "she", "or", "an", "will", "my", "one", "all", "would", "there", "their", "what", "so", "up", "out", "if", "about", "who", "get", "which", "go", "me"]

for i in 1:length(words) - 1
    word1 = words[i]
    word2 = words[i + 1]

    word1 = replace(lowercase(word1), r"[^A-Za-z]" => "")
    word2 = replace(lowercase(word2), r"[^A-Za-z]" => "")

    if word1 in stop_words || word2 in stop_words
        continue
    end

    if haskey(bigram_counts, (word1, word2))
        bigram_counts[(word1, word2)] += 1
    else
        bigram_counts[(word1, word2)] = 1
    end

    if haskey(word_counts, word1)
        word_counts[word1] += 1
    else
        word_counts[word1] = 1
    end

end

# Convert dict to array of tuples and sort by second value (frequency)
sorted_bigram_counts = sort(collect(bigram_counts), by = x -> x[2], rev = true)
sorted_word_counts = sort(collect(word_counts), by = x -> x[2], rev = true)

println("Unique words: ", length(sorted_word_counts))
println("Unique bigrams: ", length(sorted_bigram_counts), "\n")

println("Top 10 most frequent individual words:")
for (i, (word, count)) in enumerate(sorted_word_counts[1:10])
    println("$i. $word: $count")
end

println("")

println("Top 10 most frequent bigrams:")
for (i, ((word1, word2), count)) in enumerate(sorted_bigram_counts[1:10])
    println("$i. $word1 $word2: $count")
end


#=
Unique words: 2236
Unique bigrams: 6281

Top 10 most frequent individual words:
1. i: 300
2. thou: 170
3. prospero: 99
4. your: 99
5. thy: 96
6. no: 79
7. is: 75
8. ariel: 68
9. stephano: 58
10. now: 57

Top 10 most frequent bigrams:
1. i am: 33
2. no more: 14
3. thou art: 13
4. thou hast: 11
5. when i: 11
6. thou shalt: 10
7. i must: 10
8. thou didst: 9
9. thou dost: 9
10. tempest act: 9
=#