#include <vector>
#include <iostream>

using namespace std;

class ProblemFindMedianIn2SortedArrays
{
public:
    double solve(const vector<int> &A, const vector<int> &B)
    {
        const int m = A.size();
        const int n = B.size();

        if ((m + n) & 0x1)
        {
            return find_kth_item(A.begin(), m, B.begin(), n, (m + n) / 2 + 1);
        }
        else
        {
            return 0.5*(find_kth_item(A.begin(), m, B.begin(), n, (m + n) / 2) + find_kth_item(A.begin(), m, B.begin(), n, (m + n) / 2 + 1));
        }
            
    }

private:
    static int find_kth_item(std::vector<int>::const_iterator iterA, int m,
                        std::vector<int>::const_iterator iterB, int n, int k)
    {
        // if size of A is larger than B, swap them
        if (m > n)
        {
            return find_kth_item(iterB, n, iterA, m, k);
        }
        if (m == 0)
        {
            return *(iterB + k - 1);
        }
        if (k == 1)
        {
            return min(*iterA, *iterB);
        }
        // divide k into two parts
        int ia = min(k / 2, m), ib = k - ia;
        if (*(iterA + ia - 1) < *(iterB + ib - 1))
        {
            return find_kth_item(iterA + ia, m - ia, iterB, n, k - ia);
        }
        else if (*(iterA + ia - 1) > *(iterB + ib - 1))
        {
            return find_kth_item(iterA, m, iterB + ib, n - ib, k - ib);
        }
        else
        {
            return iterA[ia - 1];
        }
    }
};

int main()
{
    // input array are sorted
    vector<int> a = {-1, 2, 4, 6, 10, 11};
    vector<int> b = {-10, -4, -2, 3, 5};

    ProblemFindMedianIn2SortedArrays S;
    auto res = S.solve(a, b);
    cout << res << endl;

    return 0;
}