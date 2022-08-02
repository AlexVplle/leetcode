int* twoSum(int* nums, int numsSize, int target, int* returnSize){
	int *tab;
	if (!(tab = malloc(sizeof(int) * 2)))
		return ((void *) 0);
	for (int i = 0; i < numsSize - 1; i++){
		for (int j = i + 1; j < numsSize; j++){
			if (nums[i] + nums[j] == target){
				tab[0] = i;
				tab[1] = j;
				*returnSize = 2;
				return (tab);
			}
		}
    }
    return (tab);
}
