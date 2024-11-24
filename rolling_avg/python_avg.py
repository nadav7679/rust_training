import sys

from typing import List


def main(nums: List[float], k: int) -> List[float]:
    """
    Returns the 'k' moving average of 'nums', e.g.
        $$
            res[n] = \frac{1}{k} \sum_{i=n}^{n + k - 1} nums[i]
        $$
    where $0 <= n <= len(nums) - k$ and the sum's enpoints are inclusive.

    Assume input is valid

    :param nums: a list of floats to average.
    :param k: width of rolling window, must be lesser equal to len(nums).
    :return: a list of length (len(nums) - k + 1) with containing the averages.
    """
    

    # Algorithm
    n = len(nums)
    k_average = [0] * (n - k + 1)
    k_average[0] = sum(nums[:k]) / k  # initilize

    prev = k_average[0]
    for i in range(1, n - k + 1):
        prev += (nums[i + k - 1] - nums[i - 1]) / k
        k_average[i] = prev

    return k_average

if __name__ == "__main__":
    args = sys.argv[1:]
    k = int(args[0])
    args = [float(i) for i in args[1:]]
