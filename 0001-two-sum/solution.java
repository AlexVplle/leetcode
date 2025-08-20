import java.util.Set;
import java.util.HashMap;

class Solution {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> seen = new HashMap<>();
        int n = nums.length;
        for (int i = 0; i < n; i++) {
            int value = target - nums[i];
            if (seen.containsKey(value)) {
                return new int[]{seen.get(value), i};
            }
            seen.put(nums[i], i);
        }
        return new int[2];
    }
}
