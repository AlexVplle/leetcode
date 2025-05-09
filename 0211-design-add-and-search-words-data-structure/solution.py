class Node:
    def __init__(self):
        self.hashmap = {}
        self.end_word = False
    
    def add_child(self, char):
        new_node = Node()
        self.hashmap[char] = new_node
        return new_node

class WordDictionary:

    def __init__(self):
        self.root = Node()    

    def addWord(self, word: str) -> None:
        current_node = self.root
        for char in word:
            child = current_node.hashmap.get(char)
            if child is None:
                current_node = current_node.add_child(char)
            else:
                current_node = child
        current_node.end_word = True

    def search(self, word: str) -> bool:
        return self.find(word, self.root)
    
    def find(self, word, node) -> bool:
        if len(word) == 0:
            return node.end_word
        if word[0] == '.':
            return any([self.find(word[1:], new_node) for new_node in node.hashmap.values()])
        child = node.hashmap.get(word[0])
        if child is None:
            return False
        return self.find(word[1:], child)


# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary()
# obj.addWord(word)
# param_2 = obj.search(word)
