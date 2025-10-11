// Algo: SPF (Used to find prime factorization of numbers)
#include <bits/stdc++.h>
using namespace std;
#define int long long

const int N=1e6; // Global const for the max number whose prime factorization can be found

vector<int> largest_factor(N+10); // Vector to store largest factor of each number
void spf()
{
    for(int i=2;i<=N;i++)
        if(largest_factor[i]==0)
            for(int j=i;j<=N;j+=i)
                largest_factor[j]=i;
}
// Time Complexity: O(nlog(log(n)))

vector<int> get_factors(int n) // Function to return prime factorization of a number in a vector
{
    vector<int> prime_factors;
    while(n!=1)
    {
        prime_factors.push_back(largest_factor[n]);
        n/=largest_factor[n];
    }
    reverse(prime_factors.begin(), prime_factors.end());
    return prime_factors;
}
// Time Complexity: O(logn)

int32_t main()
{
    spf();
    // Example
    int n;
    cin >> n;
    vector<int> eg=get_factors(n);
    for(int &x:eg)
        cout << x << " ";
    return 0;
}