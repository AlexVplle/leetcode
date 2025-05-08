class Node:
    def __init__(self, isCompleteWord):
        self.hashmap = {}
        self.isCompleteWord = isCompleteWord
    
    def add_child(self, char, isCompleteWord):
        new_node = Node(isCompleteWord)
        self.hashmap[char] = new_node
        return new_node
        

class Trie:

    def __init__(self):
        self.root = Node(False)

    def insert(self, word: str) -> None:
        current_node = self.root
        for char in word:
            child = current_node.hashmap.get(char)
            if child is None:
                current_node = current_node.add_child(char, False)
            else:
                current_node = child
        current_node.isCompleteWord = True

    def search(self, word: str) -> bool:
        current_node = self.root
        for char in word:
            child = current_node.hashmap.get(char)
            if child is None:
                return False
            current_node = child
        return current_node.isCompleteWord

    def startsWith(self, prefix: str) -> bool:
        current_node = self.root
        for char in prefix:
            child = current_node.hashmap.get(char)
            if child is None:
                return False
            current_node = child
        return True


# Your Trie object will be instantiated and called as such:
# obj = Trie()
# obj.insert(word)
# param_2 = obj.search(word)
# param_3 = obj.startsWith(prefix)
