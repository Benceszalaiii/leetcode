public class Solution {
    public int CountPrimes(int n) {
        if (n < 2){
            return 0;
        }
        bool[] vec = Enumerable.Repeat(true, n).ToArray();
        vec[0] = false; vec[1] = false;
        int lastIndex = (int)Math.Floor(Math.Sqrt(vec.Length));
        for (int i = 2; i <= lastIndex; i++)
        {
            if (vec[i])
            {
                for (int j = i * 2; j < n; j += i)
                {
                    vec[j] = false;
                }
            }
        }
        return vec.Where(c => c).Count();
    }
}