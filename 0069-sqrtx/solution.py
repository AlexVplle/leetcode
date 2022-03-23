class Solution:
    def binarySearch(self, array : list, target : int) :
        end = len(array) - 1
        mid = end // 2
        if not array : return -1
        valueMid = array[mid]*array[mid]
        if valueMid == target: return mid
        if len(array) == 1 :
            if array[0]*array[0] < target : return 0
            return -1
        if valueMid > target : return self.binarySearch(array[:mid], target)
        else : return mid + 1 + self.binarySearch(array[mid+1:], target)
        
    def mySqrt(self, x: int) -> int:
        return self.binarySearch([i for i in range(46430)], x)

