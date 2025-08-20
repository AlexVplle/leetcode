import java.util.HashSet;
import java.util.Set;

class Solution {
    public boolean containsDuplicate(int[] nums) {
        Set<Integer> hashset = new HashSet<>();
        for (int value: nums) {
            if (!hashset.add(value)) {
                return true;
            }
        }
        return false;
    }
}
