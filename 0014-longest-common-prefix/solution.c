char * longestCommonPrefix(char ** strs, int strsSize){
	int	letter = 0;
	char	common;
	while (strs[0][letter])
	{
		common = strs[0][letter];
		for (int word = 0; word < strsSize; word++)
		{
			if (common != strs[word][letter])
			{
				strs[0][letter] = '\0';
				return (strs[0]);
			}
		}
		letter++;
	}
	return (strs[0]);
}

