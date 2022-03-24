class Solution:
    def mySqrt(self, x: int) -> int:
        xk = 46340
        while True :
            xkNew = xk - (xk*xk - x)/(2*xk)
            if xk - xkNew <= 1 : return int(xkNew)
            xk = xkNew

