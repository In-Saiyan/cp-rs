// Algo: Seive of Eratosthens
#include <bits/stdc++.h>
using namespace std;
#define int long long

const int N=1e6; // Global const for the max number which can be checked using sieve 

vector<bool> is_prime(N+1, true);  // Bool vector to check if a number at given index is prime
void seive()
{
    is_prime[0]=is_prime[1]=false;
    for(int i=2;i<=N;i++)
        if(is_prime[i] && i*i<=N)
            for(int j=i*i;j<=N;j+=i)
                is_prime[j]=false;
}
// Time Complexity: O(nlog(log(n)))

int32_t main()
{
    seive();
    return 0;
}