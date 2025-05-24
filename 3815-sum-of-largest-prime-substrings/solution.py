class Solution:
    def sumOfLargestPrimes(self, s: str) -> int:
        prime_numbers = set()
        n = len(s)
        def is_prime(num):
            if num <= 1:
                return False
            for i in range(2, int(num**0.5) + 1):
                if num % i == 0:
                    return False
            return True
        for i in range(n):
            for j in range(i, n):
                num_int = int(s[i:j + 1])
                if is_prime(num_int):
                    prime_numbers.add(num_int)
        prime_numbers = list(prime_numbers)
        prime_numbers.sort()
        if len(prime_numbers) > 3:
            return sum(prime_numbers[-3:])
        if len(prime_numbers) == 0:
            return 0
        return sum(prime_numbers)
