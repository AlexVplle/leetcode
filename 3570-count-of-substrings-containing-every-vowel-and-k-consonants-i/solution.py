class Solution:
    def countOfSubstrings(self, word: str, k: int) -> int:
        def f(k: int) -> int:
            vowels = "aeiou"
            counter = Counter()
            result = 0
            left = 0
            count_consonants = 0
            for char in word:
                if char in vowels:
                    counter[char] += 1
                else:
                    count_consonants += 1
                while count_consonants >= k and len(counter) == 5:
                    last_char = word[left]
                    if last_char in vowels:
                        counter[last_char] -= 1
                        if counter[last_char] == 0:
                            counter.pop(last_char)
                    else:
                        count_consonants -= 1
                    left += 1
                result += left
            return result

        return f(k) - f(k + 1)
