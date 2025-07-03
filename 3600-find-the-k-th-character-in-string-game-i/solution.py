class Solution:
    def kthCharacter(self, k: int) -> str:
        word = "a"
        while len(word) < k:
            word += ''.join([chr((ord(i) + - 96) % 26 + 97) for i in word])
        return word[k - 1]
