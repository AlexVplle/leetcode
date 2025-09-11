class Solution:
    def sortVowels(self, s: str) -> str:
        vowels = set('aeiouAEIOU')
        vowel_iter = iter(sorted([c for c in s if c in vowels]))
        return ''.join(next(vowel_iter) if c in vowels else c for c in s)
