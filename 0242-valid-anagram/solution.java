class Solution {
    public boolean isAnagram(String s, String t) {
        int s_length = s.length();
        int t_length = t.length();
        if (s_length != t_length) {
            return false;
        }
        int[] frequencies = new int[26];
        for (int i = 0; i < s_length; i++) {
            frequencies[s.charAt(i) - 'a']++;
            frequencies[t.charAt(i) - 'a']--;
        }
        for (int frequency: frequencies) {
            if (frequency != 0) {
                return false;
            }
        }
        return true;
    }
}
