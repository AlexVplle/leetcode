class Solution:
    def reverseVowels(self, s: str) -> str:
        vowels = set('aeiouAEIOU')
        reversed_vowels = iter([c for c in s if c in vowels][::-1])
        return ''.join([next(reversed_vowels) if c in vowels else c for c in s])
