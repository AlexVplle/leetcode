int myAtoi(char * s){
	long number = 0;
	int negative = 1;
	while (*s == ' ')
		s++;
	if (*s == '-' || *s == '+')
	{
		if (*s == '-')
			negative = -1;
		s++;
	}
	while (*s >= '0' && *s <= '9')
	{
		number = number * 10 + *s - '0';
        if (negative * number > 2147483647)
            return (2147483647);
        if (negative * number < -2147483648)
            return (-2147483648);
		s++;
	}
	return ((int) negative * number);
}
