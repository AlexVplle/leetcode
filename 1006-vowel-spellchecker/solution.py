class Solution:
    def spellchecker(self, wordlist: List[str], queries: List[str]) -> List[str]:
        vowels = set('aeiouAEIOU')
        exact_words = set(wordlist)
        case_dictionary = {}
        vowel_dictionary = {}
        for word in wordlist:
            word_lower = word.lower()
            if word_lower not in case_dictionary:
                case_dictionary[word_lower] = word
            vowel_pattern = ''.join(['*' if c in vowels else c for c in word_lower])
            if vowel_pattern not in vowel_dictionary:
                vowel_dictionary[vowel_pattern] = word
        result = []
        for query in queries:
            if query in exact_words:
                result.append(query)
                continue
            query_lower = query.lower()
            if query_lower in case_dictionary:
                result.append(case_dictionary[query_lower])
                continue
            query = ''.join(['*' if c in vowels else c for c in query_lower])
            if vowel_dictionary.get(query):
                result.append(vowel_dictionary[query])
            else:
                result.append("")
        return result
