bool isPalindrome(int x){
    if (x < 0)
        return (0);
    int xPrime = x;
    unsigned int y = 0;
    while (xPrime){
        y = y * 10 + xPrime % 10;
        xPrime /= 10;
    }
    if (y == x)
        return (1);
    return (0);
}
